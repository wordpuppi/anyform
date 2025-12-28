<?php
/**
 * Global settings page for Anyform.
 */

defined('ABSPATH') || exit;

class AF_Settings {

    /**
     * Initialize settings.
     */
    public static function init() {
        add_action('admin_menu', [__CLASS__, 'add_settings_page']);
        add_action('admin_init', [__CLASS__, 'register_settings']);
        add_action('wp_ajax_af_test_email', [__CLASS__, 'ajax_test_email']);
    }

    /**
     * Handle test email AJAX request.
     */
    public static function ajax_test_email() {
        check_ajax_referer('af_test_email', 'nonce');

        if (!current_user_can('manage_options')) {
            wp_send_json(['success' => false, 'message' => __('Permission denied', 'anyform')]);
        }

        $to = get_option('admin_email');
        $subject = __('Anyform Test Email', 'anyform');
        $body = '<p>' . __('This is a test email from Anyform.', 'anyform') . '</p>'
              . '<p>' . __('If you received this, your email configuration is working correctly!', 'anyform') . '</p>';

        // Use the same email method as configured
        $method = get_option('af_email_method', 'wp_mail');
        $result = false;

        if ($method === 'api') {
            $result = AF_Email::send_test($to, $subject, $body);
        } else {
            $from_name = get_option('af_email_admin_from_name', get_bloginfo('name'));
            $from_email = get_option('af_email_admin_from_email', get_option('admin_email'));
            $headers = [
                'Content-Type: text/html; charset=UTF-8',
                "From: {$from_name} <{$from_email}>",
            ];
            $result = wp_mail($to, $subject, $body, $headers);
        }

        wp_send_json([
            'success' => $result,
            'message' => $result
                ? sprintf(__('Test email sent to %s', 'anyform'), $to)
                : __('Failed to send test email. Check your email configuration.', 'anyform'),
        ]);
    }

    /**
     * Add settings page to Anyform menu.
     */
    public static function add_settings_page() {
        add_submenu_page(
            'edit.php?post_type=af_form',
            __('Anyform Settings', 'anyform'),
            __('Settings', 'anyform'),
            'manage_options',
            'anyform-settings',
            [__CLASS__, 'render_settings_page']
        );
    }

    /**
     * Register all settings.
     */
    public static function register_settings() {
        // Email Method
        register_setting('anyform_settings', 'af_email_method', [
            'type' => 'string',
            'default' => 'wp_mail',
            'sanitize_callback' => 'sanitize_text_field',
        ]);

        // API Settings
        register_setting('anyform_settings', 'af_email_api_provider', [
            'type' => 'string',
            'default' => 'sendgrid',
            'sanitize_callback' => 'sanitize_text_field',
        ]);
        register_setting('anyform_settings', 'af_email_api_key', [
            'type' => 'string',
            'default' => '',
            'sanitize_callback' => 'sanitize_text_field',
        ]);
        register_setting('anyform_settings', 'af_email_api_endpoint', [
            'type' => 'string',
            'default' => '',
            'sanitize_callback' => 'esc_url_raw',
        ]);

        // Admin Notification
        register_setting('anyform_settings', 'af_email_admin_enabled', [
            'type' => 'boolean',
            'default' => false,
        ]);
        register_setting('anyform_settings', 'af_email_admin_to', [
            'type' => 'string',
            'default' => '',
            'sanitize_callback' => 'sanitize_text_field',
        ]);
        register_setting('anyform_settings', 'af_email_admin_subject', [
            'type' => 'string',
            'default' => 'New form submission: {form_name}',
            'sanitize_callback' => 'sanitize_text_field',
        ]);
        register_setting('anyform_settings', 'af_email_admin_from_name', [
            'type' => 'string',
            'default' => '',
            'sanitize_callback' => 'sanitize_text_field',
        ]);
        register_setting('anyform_settings', 'af_email_admin_from_email', [
            'type' => 'string',
            'default' => '',
            'sanitize_callback' => 'sanitize_email',
        ]);

        // Auto-Reply
        register_setting('anyform_settings', 'af_email_reply_enabled', [
            'type' => 'boolean',
            'default' => false,
        ]);
        register_setting('anyform_settings', 'af_email_reply_field', [
            'type' => 'string',
            'default' => 'email',
            'sanitize_callback' => 'sanitize_text_field',
        ]);
        register_setting('anyform_settings', 'af_email_reply_subject', [
            'type' => 'string',
            'default' => 'Thank you for contacting us',
            'sanitize_callback' => 'sanitize_text_field',
        ]);
        register_setting('anyform_settings', 'af_email_reply_body', [
            'type' => 'string',
            'default' => 'We received your message and will respond soon.',
            'sanitize_callback' => 'wp_kses_post',
        ]);
    }

    /**
     * Render settings page.
     */
    public static function render_settings_page() {
        if (!current_user_can('manage_options')) {
            return;
        }

        // Show save message
        if (isset($_GET['settings-updated'])) {
            add_settings_error('anyform_messages', 'anyform_message',
                __('Settings saved.', 'anyform'), 'updated');
        }

        settings_errors('anyform_messages');

        $method = get_option('af_email_method', 'wp_mail');
        $api_provider = get_option('af_email_api_provider', 'sendgrid');
        ?>
        <div class="wrap">
            <h1><?php echo esc_html(get_admin_page_title()); ?></h1>

            <form action="options.php" method="post">
                <?php settings_fields('anyform_settings'); ?>

                <!-- Email Method -->
                <h2><?php esc_html_e('Email Method', 'anyform'); ?></h2>
                <table class="form-table">
                    <tr>
                        <th scope="row"><?php esc_html_e('Send emails via', 'anyform'); ?></th>
                        <td>
                            <fieldset>
                                <label>
                                    <input type="radio" name="af_email_method" value="wp_mail"
                                        <?php checked($method, 'wp_mail'); ?>>
                                    <?php esc_html_e('WordPress wp_mail()', 'anyform'); ?>
                                    <p class="description"><?php esc_html_e('Configure SMTP via a plugin like WP Mail SMTP', 'anyform'); ?></p>
                                </label>
                                <br><br>
                                <label>
                                    <input type="radio" name="af_email_method" value="api"
                                        <?php checked($method, 'api'); ?>>
                                    <?php esc_html_e('External API', 'anyform'); ?>
                                </label>
                            </fieldset>
                        </td>
                    </tr>
                </table>

                <!-- API Settings (shown when method = api) -->
                <div id="af-api-settings" style="<?php echo $method !== 'api' ? 'display:none;' : ''; ?>">
                    <h3><?php esc_html_e('API Settings', 'anyform'); ?></h3>
                    <table class="form-table">
                        <tr>
                            <th scope="row"><?php esc_html_e('Provider', 'anyform'); ?></th>
                            <td>
                                <select name="af_email_api_provider" id="af_email_api_provider">
                                    <option value="sendgrid" <?php selected($api_provider, 'sendgrid'); ?>>SendGrid</option>
                                    <option value="mailgun" <?php selected($api_provider, 'mailgun'); ?>>Mailgun</option>
                                    <option value="custom" <?php selected($api_provider, 'custom'); ?>><?php esc_html_e('Custom API', 'anyform'); ?></option>
                                </select>
                            </td>
                        </tr>
                        <tr>
                            <th scope="row"><?php esc_html_e('API Key', 'anyform'); ?></th>
                            <td>
                                <input type="password" name="af_email_api_key" class="regular-text"
                                    value="<?php echo esc_attr(get_option('af_email_api_key')); ?>">
                            </td>
                        </tr>
                        <tr id="af-custom-endpoint" style="<?php echo $api_provider !== 'custom' ? 'display:none;' : ''; ?>">
                            <th scope="row"><?php esc_html_e('API Endpoint', 'anyform'); ?></th>
                            <td>
                                <input type="url" name="af_email_api_endpoint" class="regular-text"
                                    value="<?php echo esc_attr(get_option('af_email_api_endpoint')); ?>"
                                    placeholder="https://api.example.com/mail/send">
                            </td>
                        </tr>
                    </table>
                </div>

                <!-- Test Email -->
                <h2><?php esc_html_e('Test Email', 'anyform'); ?></h2>
                <table class="form-table">
                    <tr>
                        <th scope="row"><?php esc_html_e('Test Configuration', 'anyform'); ?></th>
                        <td>
                            <button type="button" id="af-test-email" class="button">
                                <?php esc_html_e('Send Test Email', 'anyform'); ?>
                            </button>
                            <span id="af-test-result" style="margin-left: 10px;"></span>
                            <p class="description">
                                <?php
                                // translators: %s is the admin email address
                                printf(esc_html__('Sends a test email to %s using your current settings.', 'anyform'), esc_html(get_option('admin_email')));
                                ?>
                            </p>
                        </td>
                    </tr>
                </table>

                <!-- Admin Notification -->
                <h2><?php esc_html_e('Admin Notification', 'anyform'); ?></h2>
                <table class="form-table">
                    <tr>
                        <th scope="row"><?php esc_html_e('Enable', 'anyform'); ?></th>
                        <td>
                            <label>
                                <input type="checkbox" name="af_email_admin_enabled" value="1"
                                    <?php checked(get_option('af_email_admin_enabled')); ?>>
                                <?php esc_html_e('Send email to admin on new submissions', 'anyform'); ?>
                            </label>
                        </td>
                    </tr>
                    <tr>
                        <th scope="row"><?php esc_html_e('To', 'anyform'); ?></th>
                        <td>
                            <input type="text" name="af_email_admin_to" class="regular-text"
                                value="<?php echo esc_attr(get_option('af_email_admin_to', get_option('admin_email'))); ?>"
                                placeholder="admin@example.com, sales@example.com">
                            <p class="description"><?php esc_html_e('Comma-separated list of email addresses', 'anyform'); ?></p>
                        </td>
                    </tr>
                    <tr>
                        <th scope="row"><?php esc_html_e('Subject', 'anyform'); ?></th>
                        <td>
                            <input type="text" name="af_email_admin_subject" class="regular-text"
                                value="<?php echo esc_attr(get_option('af_email_admin_subject', 'New form submission: {form_name}')); ?>">
                            <p class="description"><?php esc_html_e('Use {form_name} for the form title', 'anyform'); ?></p>
                        </td>
                    </tr>
                    <tr>
                        <th scope="row"><?php esc_html_e('From Name', 'anyform'); ?></th>
                        <td>
                            <input type="text" name="af_email_admin_from_name" class="regular-text"
                                value="<?php echo esc_attr(get_option('af_email_admin_from_name', get_bloginfo('name'))); ?>"
                                placeholder="<?php echo esc_attr(get_bloginfo('name')); ?>">
                        </td>
                    </tr>
                    <tr>
                        <th scope="row"><?php esc_html_e('From Email', 'anyform'); ?></th>
                        <td>
                            <input type="email" name="af_email_admin_from_email" class="regular-text"
                                value="<?php echo esc_attr(get_option('af_email_admin_from_email', get_option('admin_email'))); ?>"
                                placeholder="<?php echo esc_attr(get_option('admin_email')); ?>">
                        </td>
                    </tr>
                </table>

                <!-- Auto-Reply -->
                <h2><?php esc_html_e('Auto-Reply', 'anyform'); ?></h2>
                <table class="form-table">
                    <tr>
                        <th scope="row"><?php esc_html_e('Enable', 'anyform'); ?></th>
                        <td>
                            <label>
                                <input type="checkbox" name="af_email_reply_enabled" value="1"
                                    <?php checked(get_option('af_email_reply_enabled')); ?>>
                                <?php esc_html_e('Send confirmation email to form submitter', 'anyform'); ?>
                            </label>
                        </td>
                    </tr>
                    <tr>
                        <th scope="row"><?php esc_html_e('Email Field', 'anyform'); ?></th>
                        <td>
                            <input type="text" name="af_email_reply_field" class="regular-text"
                                value="<?php echo esc_attr(get_option('af_email_reply_field', 'email')); ?>">
                            <p class="description"><?php esc_html_e('Form field name containing submitter email', 'anyform'); ?></p>
                        </td>
                    </tr>
                    <tr>
                        <th scope="row"><?php esc_html_e('Subject', 'anyform'); ?></th>
                        <td>
                            <input type="text" name="af_email_reply_subject" class="regular-text"
                                value="<?php echo esc_attr(get_option('af_email_reply_subject', 'Thank you for contacting us')); ?>">
                        </td>
                    </tr>
                    <tr>
                        <th scope="row"><?php esc_html_e('Body', 'anyform'); ?></th>
                        <td>
                            <textarea name="af_email_reply_body" class="large-text" rows="5"><?php
                                echo esc_textarea(get_option('af_email_reply_body', 'We received your message and will respond soon.'));
                            ?></textarea>
                            <p class="description"><?php esc_html_e('HTML is allowed', 'anyform'); ?></p>
                        </td>
                    </tr>
                </table>

                <!-- TODO: Per-form overrides in future -->

                <?php submit_button(); ?>
            </form>
        </div>

        <script>
        jQuery(function($) {
            // Toggle API settings visibility
            $('input[name="af_email_method"]').on('change', function() {
                $('#af-api-settings').toggle($(this).val() === 'api');
            });

            // Toggle custom endpoint visibility
            $('#af_email_api_provider').on('change', function() {
                $('#af-custom-endpoint').toggle($(this).val() === 'custom');
            });

            // Test email button
            $('#af-test-email').on('click', function() {
                var $btn = $(this);
                var $result = $('#af-test-result');

                $btn.prop('disabled', true).text('<?php echo esc_js(__('Sending...', 'anyform')); ?>');
                $result.text('').css('color', '');

                $.post(ajaxurl, {
                    action: 'af_test_email',
                    nonce: '<?php echo esc_attr(wp_create_nonce('af_test_email')); ?>'
                }, function(response) {
                    $result.text(response.message).css('color', response.success ? 'green' : 'red');
                    $btn.prop('disabled', false).text('<?php echo esc_js(__('Send Test Email', 'anyform')); ?>');
                }).fail(function() {
                    $result.text('<?php echo esc_js(__('Request failed', 'anyform')); ?>').css('color', 'red');
                    $btn.prop('disabled', false).text('<?php echo esc_js(__('Send Test Email', 'anyform')); ?>');
                });
            });
        });
        </script>
        <?php
    }
}
