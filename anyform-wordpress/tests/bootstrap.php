<?php
/**
 * PHPUnit bootstrap file for Anyform plugin tests.
 */

// Composer autoloader
$composer_autoload = dirname(__DIR__) . '/vendor/autoload.php';
if (file_exists($composer_autoload)) {
    require_once $composer_autoload;
}

// Load WordPress test library
$_tests_dir = getenv('WP_TESTS_DIR');

if (!$_tests_dir) {
    $_tests_dir = rtrim(sys_get_temp_dir(), '/\\') . '/wordpress-tests-lib';
}

if (!file_exists("{$_tests_dir}/includes/functions.php")) {
    echo "Could not find WordPress test library at {$_tests_dir}\n";
    echo "Run: bash bin/install-wp-tests.sh wordpress_test root '' localhost latest\n";
    exit(1);
}

// Give access to tests_add_filter() function
require_once "{$_tests_dir}/includes/functions.php";

/**
 * Manually load the plugin being tested.
 */
function _manually_load_plugin() {
    require dirname(__DIR__) . '/anyform.php';
}
tests_add_filter('muplugins_loaded', '_manually_load_plugin');

// Start up the WP testing environment
require "{$_tests_dir}/includes/bootstrap.php";
