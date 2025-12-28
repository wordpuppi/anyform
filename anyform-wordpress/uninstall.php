<?php
/**
 * Anyform Uninstall
 *
 * This file runs when the plugin is deleted from the WordPress admin.
 * It cleans up all plugin data including database tables, options, and posts.
 */

// Exit if not called by WordPress uninstall
if (!defined('WP_UNINSTALL_PLUGIN')) {
    exit;
}

global $wpdb;

// Delete submissions table
$wpdb->query("DROP TABLE IF EXISTS {$wpdb->prefix}af_submissions");

// Delete all options
$anyform_options = [
    'af_email_method',
    'af_email_api_provider',
    'af_email_api_key',
    'af_email_api_endpoint',
    'af_email_admin_enabled',
    'af_email_admin_to',
    'af_email_admin_subject',
    'af_email_admin_from_name',
    'af_email_admin_from_email',
    'af_email_reply_enabled',
    'af_email_reply_field',
    'af_email_reply_subject',
    'af_email_reply_body',
];

foreach ($anyform_options as $anyform_option) {
    delete_option($anyform_option);
}

// Delete all form posts and their meta
$anyform_forms = get_posts([
    'post_type' => 'af_form',
    'numberposts' => -1,
    'post_status' => 'any',
]);

foreach ($anyform_forms as $anyform_form) {
    wp_delete_post($anyform_form->ID, true);
}

// Clean up any transients
$wpdb->query("DELETE FROM {$wpdb->options} WHERE option_name LIKE '_transient_af_%'");
$wpdb->query("DELETE FROM {$wpdb->options} WHERE option_name LIKE '_transient_timeout_af_%'");
