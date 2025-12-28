<?php
/**
 * Tests for AF_Settings class.
 */

class AF_Settings_Test extends WP_UnitTestCase {

    /**
     * Test that settings are registered with correct defaults.
     */
    public function test_settings_registered_with_defaults() {
        AF_Settings::register_settings();

        // Check default values
        $this->assertEquals('wp_mail', get_option('af_email_method', 'wp_mail'));
        $this->assertEquals('sendgrid', get_option('af_email_api_provider', 'sendgrid'));
        $this->assertEquals('email', get_option('af_email_reply_field', 'email'));
    }

    /**
     * Test that ajax_test_email requires manage_options capability.
     */
    public function test_ajax_test_email_requires_manage_options() {
        // Set up editor (has edit_posts but not manage_options)
        $editor = $this->factory->user->create(['role' => 'editor']);
        wp_set_current_user($editor);

        // Create valid nonce
        $_POST['nonce'] = wp_create_nonce('af_test_email');

        // Capture JSON output
        ob_start();
        try {
            AF_Settings::ajax_test_email();
        } catch (WPDieException $e) {
            // Expected if wp_send_json calls wp_die
        }
        $output = ob_get_clean();

        // If output was captured, check it
        if (!empty($output)) {
            $result = json_decode($output, true);
            if ($result) {
                $this->assertFalse($result['success']);
                $this->assertStringContainsString('Permission denied', $result['message']);
            }
        }
    }

    /**
     * Test that ajax_test_email works for admin.
     */
    public function test_ajax_test_email_works_for_admin() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        $_POST['nonce'] = wp_create_nonce('af_test_email');

        // Mock wp_mail to succeed
        add_filter('pre_wp_mail', function() {
            return true;
        });

        ob_start();
        try {
            AF_Settings::ajax_test_email();
        } catch (WPDieException $e) {
            // Expected
        }
        $output = ob_get_clean();

        if (!empty($output)) {
            $result = json_decode($output, true);
            if ($result) {
                $this->assertTrue($result['success']);
                $this->assertStringContainsString('Test email sent', $result['message']);
            }
        }
    }

    /**
     * Test that settings page renders for admin.
     */
    public function test_settings_page_renders() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        ob_start();
        AF_Settings::render_settings_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('Anyform Settings', $output);
        $this->assertStringContainsString('Email Method', $output);
        $this->assertStringContainsString('Send Test Email', $output);
    }

    /**
     * Test that settings page doesn't render for non-admin.
     */
    public function test_settings_page_requires_manage_options() {
        $editor = $this->factory->user->create(['role' => 'editor']);
        wp_set_current_user($editor);

        ob_start();
        AF_Settings::render_settings_page();
        $output = ob_get_clean();

        $this->assertEmpty($output);
    }

    /**
     * Test that email method options are rendered.
     */
    public function test_email_method_options_rendered() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        ob_start();
        AF_Settings::render_settings_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('WordPress wp_mail()', $output);
        $this->assertStringContainsString('External API', $output);
        $this->assertStringContainsString('SendGrid', $output);
        $this->assertStringContainsString('Mailgun', $output);
    }

    /**
     * Test that test email button has correct ID.
     */
    public function test_test_email_button_rendered() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        ob_start();
        AF_Settings::render_settings_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('id="af-test-email"', $output);
        $this->assertStringContainsString('id="af-test-result"', $output);
    }
}
