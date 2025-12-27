<?php
/**
 * Plugin Name: Anyform
 * Plugin URI: https://github.com/epenabella/anyform
 * Description: Dynamic forms powered by anyform-js WASM client
 * Version: 1.0.0
 * Requires at least: 6.0
 * Requires PHP: 8.0
 * Author: Anyform Team
 * License: MIT
 * Text Domain: anyform
 */

defined('ABSPATH') || exit;

define('ANYFORM_VERSION', '1.0.0');
define('ANYFORM_PATH', plugin_dir_path(__FILE__));
define('ANYFORM_URL', plugin_dir_url(__FILE__));
define('ANYFORM_JS_VERSION', '0.4.0');

// Load classes
require_once ANYFORM_PATH . 'includes/class-af-database.php';
require_once ANYFORM_PATH . 'includes/class-af-post-type.php';
require_once ANYFORM_PATH . 'includes/class-af-rest-api.php';
require_once ANYFORM_PATH . 'includes/class-af-shortcode.php';
require_once ANYFORM_PATH . 'includes/class-af-email.php';
require_once ANYFORM_PATH . 'includes/class-af-settings.php';
require_once ANYFORM_PATH . 'includes/class-af-submissions.php';

/**
 * Plugin activation.
 */
function anyform_activate() {
    AF_Database::create_tables();
    AF_Post_Type::register();
    flush_rewrite_rules();
}
register_activation_hook(__FILE__, 'anyform_activate');

/**
 * Plugin deactivation.
 */
function anyform_deactivate() {
    flush_rewrite_rules();
}
register_deactivation_hook(__FILE__, 'anyform_deactivate');

/**
 * Initialize plugin.
 */
function anyform_init() {
    AF_Post_Type::register();
    AF_Settings::init();
    AF_Submissions::init();
}
add_action('init', 'anyform_init');

/**
 * Register REST API routes.
 */
function anyform_rest_init() {
    $api = new AF_REST_API();
    $api->register_routes();
}
add_action('rest_api_init', 'anyform_rest_init');

/**
 * Register shortcode.
 */
function anyform_register_shortcode() {
    $shortcode = new AF_Shortcode();
    add_shortcode('anyform', [$shortcode, 'render']);
}
add_action('init', 'anyform_register_shortcode');

/**
 * Enqueue admin styles.
 */
function anyform_admin_styles($hook) {
    global $post_type;
    if ($post_type === 'af_form') {
        wp_enqueue_style(
            'anyform-admin',
            ANYFORM_URL . 'assets/css/anyform-admin.css',
            [],
            ANYFORM_VERSION
        );
    }
}
add_action('admin_enqueue_scripts', 'anyform_admin_styles');
