<?php
/**
 * Custom Post Type for Anyform forms.
 */

defined('ABSPATH') || exit;

class AF_Post_Type {

    /**
     * Register the af_form post type.
     */
    public static function register() {
        register_post_type('af_form', [
            'labels' => [
                'name' => __('Forms', 'anyform'),
                'singular_name' => __('Form', 'anyform'),
                'add_new' => __('Add New Form', 'anyform'),
                'add_new_item' => __('Add New Form', 'anyform'),
                'edit_item' => __('Edit Form', 'anyform'),
                'new_item' => __('New Form', 'anyform'),
                'view_item' => __('View Form', 'anyform'),
                'search_items' => __('Search Forms', 'anyform'),
                'not_found' => __('No forms found', 'anyform'),
                'not_found_in_trash' => __('No forms found in trash', 'anyform'),
                'menu_name' => __('Anyform', 'anyform'),
            ],
            'public' => false,
            'show_ui' => true,
            'show_in_menu' => true,
            'show_in_rest' => false,
            'supports' => ['title', 'revisions'],
            'menu_icon' => 'dashicons-forms',
            'capability_type' => 'post',
            'rewrite' => false,
        ]);

        add_action('add_meta_boxes', [__CLASS__, 'add_meta_boxes']);
        add_action('save_post_af_form', [__CLASS__, 'save_settings'], 10, 2);
        add_action('admin_notices', [__CLASS__, 'show_json_errors']);
    }

    /**
     * Add meta boxes.
     */
    public static function add_meta_boxes() {
        add_meta_box(
            'af_form_json',
            __('Form JSON', 'anyform'),
            [__CLASS__, 'render_json_meta_box'],
            'af_form',
            'normal',
            'high'
        );

        add_meta_box(
            'af_form_settings',
            __('Form Settings', 'anyform'),
            [__CLASS__, 'render_settings_meta_box'],
            'af_form',
            'side',
            'default'
        );

        add_meta_box(
            'af_form_shortcode',
            __('Shortcode', 'anyform'),
            [__CLASS__, 'render_shortcode_meta_box'],
            'af_form',
            'side',
            'high'
        );
    }

    /**
     * Render JSON editor meta box.
     */
    public static function render_json_meta_box($post) {
        $json = get_post_meta($post->ID, 'af_form_json', true) ?: '';
        wp_nonce_field('af_form_json', 'af_form_json_nonce');
        ?>
        <p>
            <label for="af_form_json"><?php esc_html_e('Paste your form JSON schema below:', 'anyform'); ?></label>
        </p>
        <textarea id="af_form_json" name="af_form_json" rows="20"
            style="width:100%;font-family:monospace;font-size:13px;"
            placeholder='{"steps":[{"name":"Step 1","fields":[...]}]}'><?php echo esc_textarea($json); ?></textarea>
        <p class="description">
            <?php esc_html_e('Form schema in JSON format. Must contain a "steps" array with fields.', 'anyform'); ?>
        </p>
        <?php
    }

    /**
     * Render settings meta box.
     */
    public static function render_settings_meta_box($post) {
        $settings = get_post_meta($post->ID, 'af_form_settings', true) ?: [];
        ?>
        <p>
            <label for="af_action_url"><strong><?php esc_html_e('Action URL', 'anyform'); ?></strong></label><br>
            <input type="url" id="af_action_url" name="af_settings[action_url]"
                value="<?php echo esc_attr($settings['action_url'] ?? ''); ?>"
                class="widefat" placeholder="https://webhook.site/xxx">
            <span class="description"><?php esc_html_e('Optional: Forward submissions to external URL', 'anyform'); ?></span>
        </p>
        <p>
            <label for="af_success_message"><strong><?php esc_html_e('Success Message', 'anyform'); ?></strong></label><br>
            <input type="text" id="af_success_message" name="af_settings[success_message]"
                value="<?php echo esc_attr($settings['success_message'] ?? ''); ?>"
                class="widefat" placeholder="Thank you for your submission!">
        </p>
        <?php
    }

    /**
     * Render shortcode meta box.
     */
    public static function render_shortcode_meta_box($post) {
        if ($post->post_status === 'auto-draft') {
            echo '<p>' . esc_html__('Save the form first to get the shortcode.', 'anyform') . '</p>';
            return;
        }
        $slug = $post->post_name ?: sanitize_title($post->post_title);
        ?>
        <code style="display:block;padding:10px;background:#f0f0f0;word-break:break-all;">
            [anyform slug="<?php echo esc_attr($slug); ?>"]
        </code>
        <?php
    }

    /**
     * Save form settings.
     */
    public static function save_settings($post_id, $post) {
        if (defined('DOING_AUTOSAVE') && DOING_AUTOSAVE) {
            return;
        }

        if (!current_user_can('edit_post', $post_id)) {
            return;
        }

        // Save JSON
        if (isset($_POST['af_form_json_nonce']) &&
            wp_verify_nonce(sanitize_text_field(wp_unslash($_POST['af_form_json_nonce'])), 'af_form_json')) {
            $json = isset($_POST['af_form_json']) ? wp_unslash($_POST['af_form_json']) : '';
            // Validate it's valid JSON
            if (!empty($json)) {
                $decoded = json_decode($json);
                if (json_last_error() !== JSON_ERROR_NONE) {
                    // Store error for display
                    set_transient('af_json_error_' . $post_id, json_last_error_msg(), 30);
                    // Don't save invalid JSON
                } else {
                    // Valid JSON - save it (already unslashed above)
                    update_post_meta($post_id, 'af_form_json', $json);
                    // Clear any previous error
                    delete_transient('af_json_error_' . $post_id);
                }
            } else {
                delete_post_meta($post_id, 'af_form_json');
                delete_transient('af_json_error_' . $post_id);
            }
        }

        // Save settings
        if (isset($_POST['af_form_json_nonce']) &&
            wp_verify_nonce(sanitize_text_field(wp_unslash($_POST['af_form_json_nonce'])), 'af_form_json')) {
            $settings = [];
            if (isset($_POST['af_settings']['action_url'])) {
                $settings['action_url'] = esc_url_raw(wp_unslash($_POST['af_settings']['action_url']));
            }
            if (isset($_POST['af_settings']['success_message'])) {
                $settings['success_message'] = sanitize_text_field(wp_unslash($_POST['af_settings']['success_message']));
            }
            update_post_meta($post_id, 'af_form_settings', $settings);
        }
    }

    /**
     * Get form by slug.
     */
    public static function get_by_slug($slug) {
        return get_page_by_path($slug, OBJECT, 'af_form');
    }

    /**
     * Get form JSON.
     */
    public static function get_json($post) {
        $json_string = get_post_meta($post->ID, 'af_form_json', true);
        return $json_string ? json_decode($json_string, true) : null;
    }

    /**
     * Show JSON validation errors as admin notices.
     */
    public static function show_json_errors() {
        global $post;

        if (!$post || $post->post_type !== 'af_form') {
            return;
        }

        $error = get_transient('af_json_error_' . $post->ID);
        if ($error) {
            delete_transient('af_json_error_' . $post->ID);
            echo '<div class="notice notice-error"><p><strong>' . esc_html__('JSON Error:', 'anyform') . '</strong> '
                . esc_html($error) . '</p><p>'
                . esc_html__('Your JSON was not saved. Please fix the error and try again.', 'anyform')
                . '</p></div>';
        }
    }
}
