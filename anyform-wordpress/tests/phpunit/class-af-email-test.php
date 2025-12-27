<?php
/**
 * Tests for AF_Email class.
 */

class AF_Email_Test extends WP_UnitTestCase {

    public function set_up() {
        parent::set_up();

        // Reset options
        delete_option('af_email_admin_enabled');
        delete_option('af_email_reply_enabled');
        delete_option('af_email_reply_field');
    }

    /**
     * Test that admin notification disabled returns null.
     */
    public function test_send_admin_notification_disabled_returns_null() {
        update_option('af_email_admin_enabled', false);

        $result = AF_Email::send_admin_notification(
            $this->create_mock_form(),
            ['email' => 'test@test.com']
        );

        $this->assertNull($result);
    }

    /**
     * Test that admin notification enabled attempts to send.
     */
    public function test_send_admin_notification_enabled_attempts_send() {
        update_option('af_email_admin_enabled', true);
        update_option('af_email_admin_to', 'admin@test.com');

        // Track if wp_mail is called
        $mail_sent = false;
        add_filter('pre_wp_mail', function($null, $atts) use (&$mail_sent) {
            $mail_sent = true;
            return true; // Pretend success
        }, 10, 2);

        $result = AF_Email::send_admin_notification(
            $this->create_mock_form(),
            ['email' => 'user@test.com', 'name' => 'John']
        );

        $this->assertTrue($mail_sent);
    }

    /**
     * Test that auto-reply disabled returns null.
     */
    public function test_send_auto_reply_disabled_returns_null() {
        update_option('af_email_reply_enabled', false);

        $result = AF_Email::send_auto_reply(
            $this->create_mock_form(),
            ['email' => 'test@test.com']
        );

        $this->assertNull($result);
    }

    /**
     * Test that auto-reply with no email field returns null.
     */
    public function test_send_auto_reply_no_email_field_returns_null() {
        update_option('af_email_reply_enabled', true);
        update_option('af_email_reply_field', 'email');

        $result = AF_Email::send_auto_reply(
            $this->create_mock_form(),
            ['name' => 'John'] // No email field
        );

        $this->assertNull($result);
    }

    /**
     * Test that auto-reply with invalid email returns null.
     */
    public function test_send_auto_reply_invalid_email_returns_null() {
        update_option('af_email_reply_enabled', true);
        update_option('af_email_reply_field', 'email');

        $result = AF_Email::send_auto_reply(
            $this->create_mock_form(),
            ['email' => 'not-an-email']
        );

        $this->assertNull($result);
    }

    /**
     * Test that auto-reply with valid email attempts to send.
     */
    public function test_send_auto_reply_valid_email_attempts_send() {
        update_option('af_email_reply_enabled', true);
        update_option('af_email_reply_field', 'email');
        update_option('af_email_reply_subject', 'Thanks!');
        update_option('af_email_reply_body', 'We got your message.');

        $mail_sent = false;
        add_filter('pre_wp_mail', function($null, $atts) use (&$mail_sent) {
            $mail_sent = true;
            return true;
        }, 10, 2);

        $result = AF_Email::send_auto_reply(
            $this->create_mock_form(),
            ['email' => 'valid@email.com']
        );

        $this->assertTrue($mail_sent);
    }

    /**
     * Test template variable parsing in subject.
     */
    public function test_template_variables_parsed() {
        update_option('af_email_admin_enabled', true);
        update_option('af_email_admin_to', 'admin@test.com');
        update_option('af_email_admin_subject', 'New submission from {name} for {form_name}');

        $captured_subject = '';
        add_filter('pre_wp_mail', function($null, $atts) use (&$captured_subject) {
            $captured_subject = $atts['subject'];
            return true;
        }, 10, 2);

        AF_Email::send_admin_notification(
            $this->create_mock_form(),
            ['name' => 'Alice', 'email' => 'alice@test.com']
        );

        $this->assertStringContainsString('Alice', $captured_subject);
        $this->assertStringContainsString('Test Form', $captured_subject);
    }

    /**
     * Helper: Create a mock form object.
     */
    private function create_mock_form() {
        $form = new stdClass();
        $form->ID = 1;
        $form->post_title = 'Test Form';
        $form->post_name = 'test-form';
        return $form;
    }
}
