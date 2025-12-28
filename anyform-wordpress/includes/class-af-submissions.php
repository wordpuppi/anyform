<?php
/**
 * Submissions admin page for Anyform.
 */

defined('ABSPATH') || exit;

class AF_Submissions {

    /**
     * Initialize the submissions page.
     */
    public static function init() {
        add_action('admin_menu', [__CLASS__, 'add_menu_page']);
        add_action('admin_init', [__CLASS__, 'handle_actions']);
    }

    /**
     * Add submissions page to Anyform menu.
     */
    public static function add_menu_page() {
        add_submenu_page(
            'edit.php?post_type=af_form',
            __('Submissions', 'anyform'),
            __('Submissions', 'anyform'),
            'edit_posts',
            'anyform-submissions',
            [__CLASS__, 'render_page']
        );
    }

    /**
     * Handle bulk actions and single actions.
     */
    public static function handle_actions() {
        if (!isset($_GET['page']) || $_GET['page'] !== 'anyform-submissions') {
            return;
        }

        // Single delete
        if (isset($_GET['action']) && $_GET['action'] === 'delete' && isset($_GET['submission'])) {
            check_admin_referer('af_delete_submission');
            self::delete_submission((int) $_GET['submission']);
            wp_redirect(remove_query_arg(['action', 'submission', '_wpnonce']));
            exit;
        }

        // Bulk delete
        if (isset($_POST['action']) && $_POST['action'] === 'delete' && !empty($_POST['submissions'])) {
            check_admin_referer('af_bulk_submissions');
            foreach ((array) $_POST['submissions'] as $id) {
                self::delete_submission((int) $id);
            }
            wp_redirect(remove_query_arg(['action']));
            exit;
        }
    }

    /**
     * Delete a submission.
     */
    private static function delete_submission($id) {
        global $wpdb;
        $wpdb->delete(
            $wpdb->prefix . 'af_submissions',
            ['id' => $id],
            ['%d']
        );
    }

    /**
     * Render the submissions page.
     */
    public static function render_page() {
        if (!current_user_can('edit_posts')) {
            return;
        }

        // Get all forms for filter dropdown
        $forms = get_posts([
            'post_type' => 'af_form',
            'posts_per_page' => -1,
            'post_status' => 'any',
        ]);

        // Current filter
        $filter_form = isset($_GET['form_id']) ? (int) $_GET['form_id'] : 0;

        // Get submissions
        $submissions = self::get_submissions($filter_form);

        // View modal data
        $view_submission = null;
        if (isset($_GET['view']) && is_numeric($_GET['view'])) {
            $view_submission = self::get_submission((int) $_GET['view']);
        }
        ?>
        <div class="wrap">
            <h1><?php esc_html_e('Form Submissions', 'anyform'); ?></h1>

            <!-- Filter by form -->
            <form method="get" style="margin: 1rem 0;">
                <input type="hidden" name="post_type" value="af_form">
                <input type="hidden" name="page" value="anyform-submissions">
                <label for="form_id"><?php esc_html_e('Filter by form:', 'anyform'); ?></label>
                <select name="form_id" id="form_id">
                    <option value="0"><?php esc_html_e('All forms', 'anyform'); ?></option>
                    <?php foreach ($forms as $form): ?>
                        <option value="<?php echo esc_attr($form->ID); ?>" <?php selected($filter_form, $form->ID); ?>>
                            <?php echo esc_html($form->post_title); ?>
                        </option>
                    <?php endforeach; ?>
                </select>
                <button type="submit" class="button"><?php esc_html_e('Filter', 'anyform'); ?></button>
            </form>

            <?php if (empty($submissions)): ?>
                <p><?php esc_html_e('No submissions found.', 'anyform'); ?></p>
            <?php else: ?>
                <form method="post">
                    <?php wp_nonce_field('af_bulk_submissions'); ?>
                    <div class="tablenav top">
                        <div class="alignleft actions bulkactions">
                            <select name="action">
                                <option value="-1"><?php esc_html_e('Bulk actions', 'anyform'); ?></option>
                                <option value="delete"><?php esc_html_e('Delete', 'anyform'); ?></option>
                            </select>
                            <button type="submit" class="button action"><?php esc_html_e('Apply', 'anyform'); ?></button>
                        </div>
                    </div>

                    <table class="wp-list-table widefat fixed striped">
                        <thead>
                            <tr>
                                <td class="manage-column column-cb check-column">
                                    <input type="checkbox" id="cb-select-all">
                                </td>
                                <th><?php esc_html_e('ID', 'anyform'); ?></th>
                                <th><?php esc_html_e('Form', 'anyform'); ?></th>
                                <th><?php esc_html_e('Email', 'anyform'); ?></th>
                                <th><?php esc_html_e('Date', 'anyform'); ?></th>
                                <th><?php esc_html_e('Actions', 'anyform'); ?></th>
                            </tr>
                        </thead>
                        <tbody>
                            <?php foreach ($submissions as $sub): ?>
                                <?php
                                $data = json_decode($sub->data, true) ?: [];
                                $email = $data['email'] ?? $data['Email'] ?? '';
                                $form_title = get_the_title($sub->form_id) ?: esc_html__('(deleted)', 'anyform');
                                ?>
                                <tr>
                                    <th class="check-column">
                                        <input type="checkbox" name="submissions[]" value="<?php echo esc_attr($sub->id); ?>">
                                    </th>
                                    <td><?php echo esc_html($sub->id); ?></td>
                                    <td><?php echo esc_html($form_title); ?></td>
                                    <td><?php echo esc_html($email); ?></td>
                                    <td><?php echo esc_html($sub->created_at); ?></td>
                                    <td>
                                        <a href="<?php echo esc_url(add_query_arg('view', $sub->id)); ?>" class="button button-small">
                                            <?php esc_html_e('View', 'anyform'); ?>
                                        </a>
                                        <a href="<?php echo esc_url(wp_nonce_url(add_query_arg(['action' => 'delete', 'submission' => $sub->id]), 'af_delete_submission')); ?>"
                                           class="button button-small"
                                           onclick="return confirm('<?php esc_attr_e('Delete this submission?', 'anyform'); ?>');">
                                            <?php esc_html_e('Delete', 'anyform'); ?>
                                        </a>
                                    </td>
                                </tr>
                            <?php endforeach; ?>
                        </tbody>
                    </table>
                </form>
            <?php endif; ?>

            <?php if ($view_submission): ?>
                <!-- View Modal -->
                <div id="af-view-modal" style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 100000; display: flex; align-items: center; justify-content: center;">
                    <div style="background: #fff; padding: 2rem; border-radius: 8px; max-width: 600px; width: 90%; max-height: 80vh; overflow-y: auto;">
                        <?php
                        // translators: %d is the submission ID number
                        ?>
                        <h2><?php printf(esc_html__('Submission #%d', 'anyform'), $view_submission->id); ?></h2>
                        <p><strong><?php esc_html_e('Date:', 'anyform'); ?></strong> <?php echo esc_html($view_submission->created_at); ?></p>
                        <p><strong><?php esc_html_e('IP:', 'anyform'); ?></strong> <?php echo esc_html($view_submission->ip_address ?: 'N/A'); ?></p>

                        <h3><?php esc_html_e('Data', 'anyform'); ?></h3>
                        <table class="widefat">
                            <?php
                            $data = json_decode($view_submission->data, true) ?: [];
                            foreach ($data as $key => $value):
                                if (strpos($key, '_') === 0) continue;
                            ?>
                                <tr>
                                    <th style="width: 30%;"><?php echo esc_html(ucwords(str_replace(['_', '-'], ' ', $key))); ?></th>
                                    <td><?php echo esc_html(is_array($value) ? implode(', ', $value) : $value); ?></td>
                                </tr>
                            <?php endforeach; ?>
                        </table>

                        <p style="margin-top: 1.5rem;">
                            <a href="<?php echo esc_url(remove_query_arg('view')); ?>" class="button button-primary">
                                <?php esc_html_e('Close', 'anyform'); ?>
                            </a>
                        </p>
                    </div>
                </div>
            <?php endif; ?>
        </div>
        <?php
    }

    /**
     * Get submissions with optional form filter.
     */
    private static function get_submissions($form_id = 0, $limit = 100) {
        global $wpdb;
        $table = $wpdb->prefix . 'af_submissions';

        if ($form_id > 0) {
            return $wpdb->get_results($wpdb->prepare(
                "SELECT * FROM $table WHERE form_id = %d ORDER BY created_at DESC LIMIT %d",
                $form_id,
                $limit
            ));
        }

        return $wpdb->get_results($wpdb->prepare(
            "SELECT * FROM $table ORDER BY created_at DESC LIMIT %d",
            $limit
        ));
    }

    /**
     * Get a single submission.
     */
    private static function get_submission($id) {
        global $wpdb;
        $table = $wpdb->prefix . 'af_submissions';

        return $wpdb->get_row($wpdb->prepare(
            "SELECT * FROM $table WHERE id = %d",
            $id
        ));
    }
}
