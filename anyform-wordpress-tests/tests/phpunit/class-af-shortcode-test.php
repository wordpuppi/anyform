<?php
/**
 * Tests for AF_Shortcode class.
 */

class AF_Shortcode_Test extends WP_UnitTestCase {

    private $shortcode;

    public function set_up() {
        parent::set_up();
        $this->shortcode = new AF_Shortcode();
    }

    /**
     * Test that missing slug shows error for admin users.
     */
    public function test_render_without_slug_shows_error_for_admin() {
        $admin_id = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin_id);

        $output = $this->shortcode->render([]);

        $this->assertStringContainsString('af-admin-error', $output);
        $this->assertStringContainsString('slug attribute required', $output);
    }

    /**
     * Test that missing slug returns empty for visitors.
     */
    public function test_render_without_slug_returns_empty_for_visitor() {
        wp_set_current_user(0);

        $output = $this->shortcode->render([]);

        $this->assertEmpty($output);
    }

    /**
     * Test that invalid slug shows error for admin users.
     */
    public function test_render_invalid_slug_shows_error_for_admin() {
        $editor_id = $this->factory->user->create(['role' => 'editor']);
        wp_set_current_user($editor_id);

        $output = $this->shortcode->render(['slug' => 'nonexistent-form']);

        $this->assertStringContainsString('af-admin-error', $output);
        $this->assertStringContainsString('form not found', $output);
        $this->assertStringContainsString('nonexistent-form', $output);
    }

    /**
     * Test that invalid slug returns empty for visitors.
     */
    public function test_render_invalid_slug_returns_empty_for_visitor() {
        wp_set_current_user(0);

        $output = $this->shortcode->render(['slug' => 'nonexistent-form']);

        $this->assertEmpty($output);
    }

    /**
     * Test that honeypot field is rendered in form HTML.
     */
    public function test_honeypot_field_is_rendered() {
        $form_id = $this->create_test_form('test-honeypot');
        wp_set_current_user(0);

        $output = $this->shortcode->render(['slug' => 'test-honeypot']);

        $this->assertStringContainsString('name="af_hp_test-honeypot"', $output);
        $this->assertStringContainsString('aria-hidden="true"', $output);
        $this->assertStringContainsString('tabindex="-1"', $output);
    }

    /**
     * Test aria-required attribute on required fields.
     */
    public function test_aria_required_on_required_fields() {
        $form_id = $this->create_test_form('test-aria');
        wp_set_current_user(0);

        $output = $this->shortcode->render(['slug' => 'test-aria']);

        $this->assertStringContainsString('aria-required="true"', $output);
    }

    /**
     * Test aria-describedby includes error div ID.
     */
    public function test_aria_describedby_includes_error_div() {
        $form_id = $this->create_test_form('test-describedby');
        wp_set_current_user(0);

        $output = $this->shortcode->render(['slug' => 'test-describedby']);

        $this->assertStringContainsString('aria-describedby=', $output);
        $this->assertStringContainsString('af-error-email', $output);
    }

    /**
     * Test error message div has role="alert".
     */
    public function test_error_message_div_has_role_alert() {
        $form_id = $this->create_test_form('test-alert');
        wp_set_current_user(0);

        $output = $this->shortcode->render(['slug' => 'test-alert']);

        $this->assertStringContainsString('role="alert"', $output);
        $this->assertStringContainsString('aria-live="polite"', $output);
    }

    /**
     * Test success message has role="alert" and aria-live="assertive".
     */
    public function test_success_message_has_role_alert() {
        $form_id = $this->create_test_form('test-success');

        $_GET['af_success'] = '1';
        $_GET['af_form'] = 'test-success';

        $output = $this->shortcode->render(['slug' => 'test-success']);

        $this->assertStringContainsString('af-success-message', $output);
        $this->assertStringContainsString('role="alert"', $output);
        $this->assertStringContainsString('aria-live="assertive"', $output);

        unset($_GET['af_success'], $_GET['af_form']);
    }

    /**
     * Helper: Create a test form with valid JSON.
     */
    private function create_test_form($slug) {
        $form_id = $this->factory->post->create([
            'post_type' => 'af_form',
            'post_name' => $slug,
            'post_status' => 'publish',
            'post_title' => 'Test Form',
        ]);

        $json = json_encode([
            'steps' => [
                [
                    'name' => 'Step 1',
                    'fields' => [
                        [
                            'name' => 'email',
                            'label' => 'Email',
                            'field_type' => 'email',
                            'validation' => ['required' => true],
                            'help_text' => 'Enter your email',
                        ],
                        [
                            'name' => 'name',
                            'label' => 'Name',
                            'field_type' => 'text',
                        ],
                    ],
                ],
            ],
        ]);

        update_post_meta($form_id, 'af_form_json', $json);

        return $form_id;
    }
}
