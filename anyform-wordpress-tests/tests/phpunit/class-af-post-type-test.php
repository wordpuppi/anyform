<?php
/**
 * Tests for AF_Post_Type class.
 */

class AF_Post_Type_Test extends WP_UnitTestCase {

    public function set_up() {
        parent::set_up();

        // Set up admin user for save operations
        $admin_id = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin_id);
    }

    /**
     * Test that valid JSON saves successfully.
     */
    public function test_valid_json_saves_successfully() {
        $form_id = $this->create_form();

        $_POST['af_form_json_nonce'] = wp_create_nonce('af_form_json');
        $_POST['af_form_json'] = '{"steps":[{"name":"Test","fields":[]}]}';

        AF_Post_Type::save_settings($form_id, get_post($form_id));

        $saved = get_post_meta($form_id, 'af_form_json', true);
        $this->assertEquals('{"steps":[{"name":"Test","fields":[]}]}', $saved);
    }

    /**
     * Test that invalid JSON is not saved.
     */
    public function test_invalid_json_not_saved() {
        $form_id = $this->create_form();

        // First save valid JSON
        update_post_meta($form_id, 'af_form_json', '{"valid":"json"}');

        $_POST['af_form_json_nonce'] = wp_create_nonce('af_form_json');
        $_POST['af_form_json'] = '{"invalid": json';

        AF_Post_Type::save_settings($form_id, get_post($form_id));

        // Original JSON should be preserved
        $saved = get_post_meta($form_id, 'af_form_json', true);
        $this->assertEquals('{"valid":"json"}', $saved);
    }

    /**
     * Test that invalid JSON sets a transient error.
     */
    public function test_invalid_json_sets_transient_error() {
        $form_id = $this->create_form();

        $_POST['af_form_json_nonce'] = wp_create_nonce('af_form_json');
        $_POST['af_form_json'] = '{"broken":';

        AF_Post_Type::save_settings($form_id, get_post($form_id));

        $error = get_transient('af_json_error_' . $form_id);
        $this->assertNotEmpty($error);
        $this->assertStringContainsString('Syntax error', $error);
    }

    /**
     * Test that valid JSON clears any previous error transient.
     */
    public function test_valid_json_clears_error_transient() {
        $form_id = $this->create_form();

        // Set an error transient
        set_transient('af_json_error_' . $form_id, 'Previous error', 30);

        $_POST['af_form_json_nonce'] = wp_create_nonce('af_form_json');
        $_POST['af_form_json'] = '{"valid":"json"}';

        AF_Post_Type::save_settings($form_id, get_post($form_id));

        $error = get_transient('af_json_error_' . $form_id);
        $this->assertFalse($error);
    }

    /**
     * Test that show_json_errors displays admin notice.
     */
    public function test_show_json_errors_displays_notice() {
        global $post;
        $form_id = $this->create_form();
        $post = get_post($form_id);

        set_transient('af_json_error_' . $form_id, 'Syntax error', 30);

        ob_start();
        AF_Post_Type::show_json_errors();
        $output = ob_get_clean();

        $this->assertStringContainsString('notice-error', $output);
        $this->assertStringContainsString('Syntax error', $output);
        $this->assertStringContainsString('JSON Error:', $output);
    }

    /**
     * Test that show_json_errors deletes the transient after displaying.
     */
    public function test_show_json_errors_deletes_transient() {
        global $post;
        $form_id = $this->create_form();
        $post = get_post($form_id);

        set_transient('af_json_error_' . $form_id, 'Error message', 30);

        ob_start();
        AF_Post_Type::show_json_errors();
        ob_end_clean();

        $error = get_transient('af_json_error_' . $form_id);
        $this->assertFalse($error);
    }

    /**
     * Test that empty JSON is allowed and deletes the meta.
     */
    public function test_empty_json_deletes_meta() {
        $form_id = $this->create_form();

        // First save some JSON
        update_post_meta($form_id, 'af_form_json', '{"some":"json"}');

        $_POST['af_form_json_nonce'] = wp_create_nonce('af_form_json');
        $_POST['af_form_json'] = '';

        AF_Post_Type::save_settings($form_id, get_post($form_id));

        $saved = get_post_meta($form_id, 'af_form_json', true);
        $this->assertEmpty($saved);
    }

    /**
     * Test get_by_slug returns correct form.
     */
    public function test_get_by_slug_returns_form() {
        $form_id = $this->factory->post->create([
            'post_type' => 'af_form',
            'post_name' => 'my-test-form',
            'post_status' => 'publish',
        ]);

        $form = AF_Post_Type::get_by_slug('my-test-form');

        $this->assertInstanceOf(WP_Post::class, $form);
        $this->assertEquals($form_id, $form->ID);
    }

    /**
     * Test get_json returns decoded JSON.
     */
    public function test_get_json_returns_decoded_array() {
        $form_id = $this->create_form();
        update_post_meta($form_id, 'af_form_json', '{"steps":[]}');

        $json = AF_Post_Type::get_json(get_post($form_id));

        $this->assertIsArray($json);
        $this->assertArrayHasKey('steps', $json);
    }

    /**
     * Helper: Create a test form.
     */
    private function create_form() {
        return $this->factory->post->create([
            'post_type' => 'af_form',
            'post_status' => 'publish',
        ]);
    }
}
