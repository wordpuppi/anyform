<?php
/**
 * Database operations for Anyform.
 */

defined('ABSPATH') || exit;

class AF_Database {

    /**
     * Create submissions table.
     */
    public static function create_tables() {
        global $wpdb;

        $table_name = $wpdb->prefix . 'af_submissions';
        $charset_collate = $wpdb->get_charset_collate();

        $sql = "CREATE TABLE $table_name (
            id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
            form_id BIGINT UNSIGNED NOT NULL,
            data JSON NOT NULL,
            ip_address VARCHAR(45) DEFAULT NULL,
            user_agent VARCHAR(512) DEFAULT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            INDEX idx_form_id (form_id),
            INDEX idx_created_at (created_at)
        ) $charset_collate;";

        require_once ABSPATH . 'wp-admin/includes/upgrade.php';
        dbDelta($sql);
    }

    /**
     * Save a form submission.
     */
    public static function save_submission($form_id, $data, $ip = null, $user_agent = null) {
        global $wpdb;

        $result = $wpdb->insert(
            $wpdb->prefix . 'af_submissions',
            [
                'form_id' => $form_id,
                'data' => wp_json_encode($data),
                'ip_address' => $ip,
                'user_agent' => $user_agent,
            ],
            ['%d', '%s', '%s', '%s']
        );

        return $result ? $wpdb->insert_id : false;
    }

    /**
     * Get submissions for a form.
     */
    public static function get_submissions($form_id, $limit = 50, $offset = 0) {
        global $wpdb;

        $table = $wpdb->prefix . 'af_submissions';

        return $wpdb->get_results(
            $wpdb->prepare(
                "SELECT * FROM $table WHERE form_id = %d ORDER BY created_at DESC LIMIT %d OFFSET %d",
                $form_id,
                $limit,
                $offset
            )
        );
    }
}
