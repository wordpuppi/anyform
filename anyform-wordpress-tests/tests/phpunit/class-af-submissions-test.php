<?php
/**
 * Tests for AF_Submissions class.
 */

class AF_Submissions_Test extends WP_UnitTestCase {

    public function set_up() {
        parent::set_up();
        AF_Database::create_tables();
    }

    /**
     * Test that render_page requires edit_posts capability.
     */
    public function test_render_page_requires_capability() {
        // Set up subscriber (no edit_posts capability)
        $subscriber = $this->factory->user->create(['role' => 'subscriber']);
        wp_set_current_user($subscriber);

        ob_start();
        AF_Submissions::render_page();
        $output = ob_get_clean();

        $this->assertEmpty($output);
    }

    /**
     * Test that render_page works for editor.
     */
    public function test_render_page_works_for_editor() {
        $editor = $this->factory->user->create(['role' => 'editor']);
        wp_set_current_user($editor);

        ob_start();
        AF_Submissions::render_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('Form Submissions', $output);
    }

    /**
     * Test empty state message.
     */
    public function test_empty_state_shows_message() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        ob_start();
        AF_Submissions::render_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('No submissions found', $output);
    }

    /**
     * Test that submissions are displayed in the list.
     */
    public function test_submissions_displayed_in_list() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        // Create a form and submission
        $form_id = $this->factory->post->create([
            'post_type' => 'af_form',
            'post_title' => 'Contact Form',
            'post_status' => 'publish',
        ]);

        AF_Database::save_submission($form_id, [
            'email' => 'test@example.com',
            'name' => 'John Doe',
        ], '127.0.0.1', 'Test Agent');

        ob_start();
        AF_Submissions::render_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('Contact Form', $output);
        $this->assertStringContainsString('test@example.com', $output);
    }

    /**
     * Test filter by form dropdown is rendered.
     */
    public function test_filter_dropdown_rendered() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        // Create a form
        $this->factory->post->create([
            'post_type' => 'af_form',
            'post_title' => 'My Form',
            'post_status' => 'publish',
        ]);

        ob_start();
        AF_Submissions::render_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('Filter by form:', $output);
        $this->assertStringContainsString('My Form', $output);
    }

    /**
     * Test view modal shows submission data.
     */
    public function test_view_modal_shows_data() {
        $admin = $this->factory->user->create(['role' => 'administrator']);
        wp_set_current_user($admin);

        $form_id = $this->factory->post->create([
            'post_type' => 'af_form',
            'post_status' => 'publish',
        ]);

        $submission_id = AF_Database::save_submission($form_id, [
            'email' => 'modal@test.com',
            'message' => 'Hello World',
        ], '192.168.1.1');

        $_GET['view'] = $submission_id;

        ob_start();
        AF_Submissions::render_page();
        $output = ob_get_clean();

        $this->assertStringContainsString('af-view-modal', $output);
        $this->assertStringContainsString('modal@test.com', $output);
        $this->assertStringContainsString('Hello World', $output);
        $this->assertStringContainsString('192.168.1.1', $output);

        unset($_GET['view']);
    }

    /**
     * Test that delete action removes submission.
     */
    public function test_delete_removes_submission() {
        global $wpdb;
        $table = $wpdb->prefix . 'af_submissions';

        $form_id = $this->factory->post->create(['post_type' => 'af_form']);
        $submission_id = AF_Database::save_submission($form_id, ['test' => 'data']);

        // Verify it exists
        $exists = $wpdb->get_var($wpdb->prepare(
            "SELECT COUNT(*) FROM $table WHERE id = %d",
            $submission_id
        ));
        $this->assertEquals(1, $exists);

        // Use reflection to call private delete method
        $reflection = new ReflectionClass('AF_Submissions');
        $method = $reflection->getMethod('delete_submission');
        $method->setAccessible(true);
        $method->invoke(null, $submission_id);

        // Verify it's gone
        $exists = $wpdb->get_var($wpdb->prepare(
            "SELECT COUNT(*) FROM $table WHERE id = %d",
            $submission_id
        ));
        $this->assertEquals(0, $exists);
    }
}
