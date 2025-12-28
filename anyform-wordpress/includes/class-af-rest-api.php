<?php
/**
 * REST API endpoints for Anyform.
 */

defined('ABSPATH') || exit;

class AF_REST_API {

    const NAMESPACE = 'anyform/v1';

    /**
     * Register REST routes.
     */
    public function register_routes() {
        // Get form schema
        register_rest_route(self::NAMESPACE, '/forms/(?P<slug>[a-z0-9-]+)', [
            [
                'methods' => WP_REST_Server::READABLE,
                'callback' => [$this, 'get_form'],
                'permission_callback' => '__return_true',
                'args' => [
                    'slug' => [
                        'required' => true,
                        'type' => 'string',
                        'sanitize_callback' => 'sanitize_title',
                    ],
                ],
            ],
            [
                'methods' => WP_REST_Server::CREATABLE,
                'callback' => [$this, 'submit_form'],
                'permission_callback' => '__return_true',
                'args' => [
                    'slug' => [
                        'required' => true,
                        'type' => 'string',
                        'sanitize_callback' => 'sanitize_title',
                    ],
                ],
            ],
        ]);

        // List submissions (admin only)
        register_rest_route(self::NAMESPACE, '/forms/(?P<slug>[a-z0-9-]+)/submissions', [
            'methods' => WP_REST_Server::READABLE,
            'callback' => [$this, 'get_submissions'],
            'permission_callback' => [$this, 'admin_permissions_check'],
            'args' => [
                'slug' => [
                    'required' => true,
                    'type' => 'string',
                ],
            ],
        ]);
    }

    /**
     * Get form schema.
     */
    public function get_form(WP_REST_Request $request) {
        $slug = $request->get_param('slug');
        $form = AF_Post_Type::get_by_slug($slug);

        if (!$form || $form->post_status !== 'publish') {
            return new WP_Error('not_found', esc_html__('Form not found', 'anyform'), ['status' => 404]);
        }

        $json = AF_Post_Type::get_json($form);
        if (!$json) {
            return new WP_Error('invalid_json', esc_html__('Invalid form JSON', 'anyform'), ['status' => 500]);
        }

        // Ensure required fields
        $json['id'] = (string) $form->ID;
        $json['name'] = $form->post_title;
        $json['slug'] = $slug;
        $json['action_url'] = rest_url(self::NAMESPACE . "/forms/{$slug}");
        $json['action_method'] = 'POST';

        // Merge in settings
        $settings = get_post_meta($form->ID, 'af_form_settings', true) ?: [];
        if (!empty($settings['action_url'])) {
            $json['action_url'] = $settings['action_url'];
        }

        return rest_ensure_response($json);
    }

    /**
     * Submit form data.
     */
    public function submit_form(WP_REST_Request $request) {
        $slug = $request->get_param('slug');
        $form = AF_Post_Type::get_by_slug($slug);

        if (!$form || $form->post_status !== 'publish') {
            return new WP_Error('not_found', esc_html__('Form not found', 'anyform'), ['status' => 404]);
        }

        // Check if this is AJAX (JSON) or browser form POST
        $content_type = $request->get_content_type();
        $is_ajax = ($content_type['value'] ?? '') === 'application/json';

        // Get submission data
        $data = $request->get_json_params();
        if (empty($data)) {
            $data = $request->get_body_params();
        }

        // Check honeypot field - if filled, silently accept but don't process
        $hp_field = 'af_hp_' . $slug;
        if (!empty($data[$hp_field])) {
            // Bot detected - return success to avoid tipping off the bot
            if (!$is_ajax) {
                $referer = $data['_wp_http_referer'] ?? wp_get_referer() ?: home_url();
                wp_redirect(add_query_arg(['af_success' => '1', 'af_form' => $slug], $referer), 303);
                exit;
            }
            return rest_ensure_response([
                'success' => true,
                'message' => esc_html__('Thank you for your submission!', 'anyform'),
            ]);
        }
        // Remove honeypot field from data
        unset($data[$hp_field]);

        // Store referer for redirect
        $referer = $data['_wp_http_referer'] ?? wp_get_referer() ?: home_url();

        // Remove WordPress internals
        unset($data['_wpnonce'], $data['af_nonce'], $data['_wp_http_referer']);

        // Save to database
        $ip = $request->get_header('X-Forwarded-For') ?: (isset($_SERVER['REMOTE_ADDR']) ? sanitize_text_field(wp_unslash($_SERVER['REMOTE_ADDR'])) : '');
        $user_agent = $request->get_header('User-Agent') ?: '';

        $submission_id = AF_Database::save_submission($form->ID, $data, $ip, $user_agent);

        if (!$submission_id) {
            if (!$is_ajax) {
                wp_die(esc_html__('Failed to save submission', 'anyform'), esc_html__('Error', 'anyform'), ['back_link' => true]);
            }
            return new WP_Error('save_failed', esc_html__('Failed to save submission', 'anyform'), ['status' => 500]);
        }

        // Forward to action_url if configured
        $settings = get_post_meta($form->ID, 'af_form_settings', true) ?: [];
        if (!empty($settings['action_url'])) {
            $this->forward_submission($settings['action_url'], $data);
        }

        // Send email notifications
        AF_Email::send_admin_notification($form, $data);
        AF_Email::send_auto_reply($form, $data);

        $success_message = !empty($settings['success_message'])
            ? $settings['success_message']
            : esc_html__('Thank you for your submission!', 'anyform');

        // For browser form POST, redirect to thank you page
        if (!$is_ajax) {
            // Build thank you URL with message
            $thank_you_url = add_query_arg([
                'af_success' => '1',
                'af_form' => $slug,
            ], $referer);

            wp_redirect($thank_you_url, 303);
            exit;
        }

        return rest_ensure_response([
            'success' => true,
            'submission_id' => $submission_id,
            'message' => $success_message,
        ]);
    }

    /**
     * Get submissions for a form.
     */
    public function get_submissions(WP_REST_Request $request) {
        $slug = $request->get_param('slug');
        $form = AF_Post_Type::get_by_slug($slug);

        if (!$form) {
            return new WP_Error('not_found', esc_html__('Form not found', 'anyform'), ['status' => 404]);
        }

        $submissions = AF_Database::get_submissions($form->ID);

        return rest_ensure_response([
            'form_id' => $form->ID,
            'form_name' => $form->post_title,
            'submissions' => array_map(function($sub) {
                return [
                    'id' => (int) $sub->id,
                    'data' => json_decode($sub->data, true),
                    'created_at' => $sub->created_at,
                ];
            }, $submissions),
        ]);
    }

    /**
     * Forward submission to external URL.
     */
    private function forward_submission($url, $data) {
        wp_remote_post($url, [
            'body' => wp_json_encode($data),
            'headers' => [
                'Content-Type' => 'application/json',
            ],
            'timeout' => 10,
            'blocking' => false, // Don't wait for response
        ]);
    }

    /**
     * Check admin permissions.
     */
    public function admin_permissions_check() {
        return current_user_can('edit_posts');
    }
}
