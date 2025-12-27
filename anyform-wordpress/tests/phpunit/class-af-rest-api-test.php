<?php
/**
 * Tests for AF_REST_API class.
 */

class AF_REST_API_Test extends WP_UnitTestCase {

    private $api;

    public function set_up() {
        parent::set_up();
        $this->api = new AF_REST_API();

        // Ensure the submissions table exists
        AF_Database::create_tables();
    }

    /**
     * Test that normal form submission saves to database.
     */
    public function test_submit_form_saves_to_database() {
        $form_id = $this->create_test_form('submit-test');

        $request = new WP_REST_Request('POST', '/anyform/v1/forms/submit-test');
        $request->set_header('Content-Type', 'application/json');
        $request->set_body(wp_json_encode([
            'email' => 'test@example.com',
            'name' => 'Test User',
        ]));
        $request->set_param('slug', 'submit-test');

        $response = $this->api->submit_form($request);
        $data = $response->get_data();

        $this->assertTrue($data['success']);
        $this->assertArrayHasKey('submission_id', $data);
        $this->assertIsInt($data['submission_id']);
    }

    /**
     * Test that honeypot filled returns success but doesn't save.
     */
    public function test_honeypot_filled_returns_success_but_no_save() {
        $form_id = $this->create_test_form('honeypot-test');

        // Get initial submission count
        global $wpdb;
        $table = $wpdb->prefix . 'af_submissions';
        $initial_count = (int) $wpdb->get_var("SELECT COUNT(*) FROM $table WHERE form_id = $form_id");

        $request = new WP_REST_Request('POST', '/anyform/v1/forms/honeypot-test');
        $request->set_header('Content-Type', 'application/json');
        $request->set_body(wp_json_encode([
            'email' => 'bot@spam.com',
            'name' => 'Spammer',
            'af_hp_honeypot-test' => 'bot-filled-this',
        ]));
        $request->set_param('slug', 'honeypot-test');

        $response = $this->api->submit_form($request);
        $data = $response->get_data();

        // Should return success (to fool the bot)
        $this->assertTrue($data['success']);

        // But no submission_id (nothing was saved)
        $this->assertArrayNotHasKey('submission_id', $data);

        // Verify database count hasn't increased
        $final_count = (int) $wpdb->get_var("SELECT COUNT(*) FROM $table WHERE form_id = $form_id");
        $this->assertEquals($initial_count, $final_count);
    }

    /**
     * Test that normal submission (empty honeypot) saves correctly.
     */
    public function test_honeypot_empty_saves_submission() {
        $form_id = $this->create_test_form('legitimate-form');

        $request = new WP_REST_Request('POST', '/anyform/v1/forms/legitimate-form');
        $request->set_header('Content-Type', 'application/json');
        $request->set_body(wp_json_encode([
            'email' => 'real@user.com',
            'name' => 'Real User',
            'af_hp_legitimate-form' => '', // Empty honeypot
        ]));
        $request->set_param('slug', 'legitimate-form');

        $response = $this->api->submit_form($request);
        $data = $response->get_data();

        $this->assertTrue($data['success']);
        $this->assertArrayHasKey('submission_id', $data);
    }

    /**
     * Test that form not found returns 404 error.
     */
    public function test_submit_form_not_found_returns_error() {
        $request = new WP_REST_Request('POST', '/anyform/v1/forms/nonexistent');
        $request->set_param('slug', 'nonexistent');

        $response = $this->api->submit_form($request);

        $this->assertInstanceOf(WP_Error::class, $response);
        $this->assertEquals('not_found', $response->get_error_code());
    }

    /**
     * Test that get_submissions requires admin permissions.
     */
    public function test_get_submissions_requires_admin() {
        wp_set_current_user(0);
        $this->assertFalse($this->api->admin_permissions_check());

        $subscriber = $this->factory->user->create(['role' => 'subscriber']);
        wp_set_current_user($subscriber);
        $this->assertFalse($this->api->admin_permissions_check());

        $editor = $this->factory->user->create(['role' => 'editor']);
        wp_set_current_user($editor);
        $this->assertTrue($this->api->admin_permissions_check());
    }

    /**
     * Helper: Create a test form.
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
                        ['name' => 'email', 'label' => 'Email', 'field_type' => 'email'],
                        ['name' => 'name', 'label' => 'Name', 'field_type' => 'text'],
                    ],
                ],
            ],
        ]);

        update_post_meta($form_id, 'af_form_json', $json);

        return $form_id;
    }
}
