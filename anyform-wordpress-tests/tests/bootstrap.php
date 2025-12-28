<?php
/**
 * PHPUnit bootstrap file for Anyform plugin tests.
 *
 * Tests are in a separate directory from the plugin.
 * Set ANYFORM_PLUGIN_DIR in phpunit.xml.dist to point to the plugin.
 */

// Composer autoloader
$anyform_composer_autoload = dirname(__DIR__) . '/vendor/autoload.php';
if (file_exists($anyform_composer_autoload)) {
    require_once $anyform_composer_autoload;
}

// Plugin directory (relative to tests directory)
$anyform_plugin_dir = defined('ANYFORM_PLUGIN_DIR')
    ? dirname(__DIR__) . '/' . ANYFORM_PLUGIN_DIR
    : dirname(__DIR__, 2) . '/anyform-wordpress';

// Load WordPress test library
$anyform_tests_dir = getenv('WP_TESTS_DIR');

if (!$anyform_tests_dir) {
    $anyform_tests_dir = rtrim(sys_get_temp_dir(), '/\\') . '/wordpress-tests-lib';
}

if (!file_exists("{$anyform_tests_dir}/includes/functions.php")) {
    echo "Could not find WordPress test library at {$anyform_tests_dir}\n";
    echo "Run: bash bin/install-wp-tests.sh wordpress_test root '' localhost latest\n";
    exit(1);
}

// Give access to tests_add_filter() function
require_once "{$anyform_tests_dir}/includes/functions.php";

/**
 * Manually load the plugin being tested.
 */
function anyform_manually_load_plugin() {
    global $anyform_plugin_dir;
    require $anyform_plugin_dir . '/anyform.php';
}
tests_add_filter('muplugins_loaded', 'anyform_manually_load_plugin');

// Start up the WP testing environment
require "{$anyform_tests_dir}/includes/bootstrap.php";
