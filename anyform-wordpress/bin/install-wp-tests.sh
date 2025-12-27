#!/usr/bin/env bash

if [ $# -lt 3 ]; then
    echo "usage: $0 <db-name> <db-user> <db-pass> [db-host] [wp-version] [skip-database-creation]"
    exit 1
fi

DB_NAME=$1
DB_USER=$2
DB_PASS=$3
DB_HOST=${4-localhost}
WP_VERSION=${5-latest}
SKIP_DB_CREATE=${6-false}

TMPDIR=${TMPDIR-/tmp}
TMPDIR=$(echo $TMPDIR | sed -e "s/\/$//")
WP_TESTS_DIR=${WP_TESTS_DIR-$TMPDIR/wordpress-tests-lib}
WP_CORE_DIR=${WP_CORE_DIR-$TMPDIR/wordpress}

download() {
    if [ $(which curl) ]; then
        curl -s "$1" > "$2";
    elif [ $(which wget) ]; then
        wget -nv -O "$2" "$1"
    fi
}

if [[ $WP_VERSION =~ ^[0-9]+\.[0-9]+\-(beta|RC)[0-9]+$ ]]; then
    WP_BRANCH=${WP_VERSION%\-*}
    WP_TESTS_TAG="branches/$WP_BRANCH"
elif [[ $WP_VERSION =~ ^[0-9]+\.[0-9]+$ ]]; then
    WP_TESTS_TAG="branches/$WP_VERSION"
elif [[ $WP_VERSION =~ [0-9]+\.[0-9]+\.[0-9]+ ]]; then
    WP_TESTS_TAG="tags/$WP_VERSION"
elif [[ $WP_VERSION == 'nightly' || $WP_VERSION == 'trunk' ]]; then
    WP_TESTS_TAG="trunk"
else
    # Assume latest
    download http://api.wordpress.org/core/version-check/1.7/ /tmp/wp-latest.json
    LATEST_VERSION=$(grep -o '"version":"[^"]*' /tmp/wp-latest.json | sed 's/"version":"//')
    if [ -z "$LATEST_VERSION" ]; then
        echo "Could not determine latest WordPress version"
        exit 1
    fi
    WP_TESTS_TAG="tags/$LATEST_VERSION"
fi

set -ex

install_wp() {
    if [ -d $WP_CORE_DIR ]; then
        return;
    fi

    mkdir -p $WP_CORE_DIR

    if [[ $WP_VERSION == 'nightly' || $WP_VERSION == 'trunk' ]]; then
        mkdir -p $TMPDIR/wordpress-nightly
        download https://wordpress.org/nightly-builds/wordpress-latest.zip $TMPDIR/wp-nightly.zip
        unzip -q $TMPDIR/wp-nightly.zip -d $TMPDIR/wordpress-nightly/
        mv $TMPDIR/wordpress-nightly/wordpress/* $WP_CORE_DIR
    else
        if [ $WP_VERSION == 'latest' ]; then
            local ARCHIVE_NAME='latest'
        else
            local ARCHIVE_NAME="wordpress-$WP_VERSION"
        fi
        download https://wordpress.org/${ARCHIVE_NAME}.tar.gz $TMPDIR/wordpress.tar.gz
        tar --strip-components=1 -zxmf $TMPDIR/wordpress.tar.gz -C $WP_CORE_DIR
    fi

    download https://raw.github.com/markoheijnen/wp-mysqli/master/db.php $WP_CORE_DIR/wp-content/db.php
}

install_test_suite() {
    # Portable in-place argument for sed
    local ioption='-i'
    if [[ $(uname -s) == 'Darwin' ]]; then
        ioption='-i.bak'
    fi

    if [ -d $WP_TESTS_DIR ]; then
        return;
    fi

    mkdir -p $WP_TESTS_DIR

    svn co --quiet https://develop.svn.wordpress.org/${WP_TESTS_TAG}/tests/phpunit/includes/ $WP_TESTS_DIR/includes
    svn co --quiet https://develop.svn.wordpress.org/${WP_TESTS_TAG}/tests/phpunit/data/ $WP_TESTS_DIR/data

    download https://develop.svn.wordpress.org/${WP_TESTS_TAG}/wp-tests-config-sample.php $WP_TESTS_DIR/wp-tests-config.php

    sed $ioption "s:dirname( __FILE__ ) . '/src/':'$WP_CORE_DIR/':" $WP_TESTS_DIR/wp-tests-config.php
    sed $ioption "s/youremptytestdbnamehere/$DB_NAME/" $WP_TESTS_DIR/wp-tests-config.php
    sed $ioption "s/yourusernamehere/$DB_USER/" $WP_TESTS_DIR/wp-tests-config.php
    sed $ioption "s/yourpasswordhere/$DB_PASS/" $WP_TESTS_DIR/wp-tests-config.php
    sed $ioption "s|localhost|${DB_HOST}|" $WP_TESTS_DIR/wp-tests-config.php
}

install_db() {
    if [ ${SKIP_DB_CREATE} = "true" ]; then
        return 0
    fi

    # Parse DB_HOST for port or socket references
    local EXTRA=""

    if [ -n "$DB_PASS" ]; then
        EXTRA=" -p$DB_PASS"
    fi

    # Create database
    if [ $(which mysql) ]; then
        mysql --user=$DB_USER --password=$DB_PASS --host=$DB_HOST -e "DROP DATABASE IF EXISTS $DB_NAME; CREATE DATABASE $DB_NAME;"
    else
        echo "mysql client not found, skipping database creation"
    fi
}

install_wp
install_test_suite
install_db
