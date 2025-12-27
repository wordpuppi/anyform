<?php
/**
 * Email sending functionality for Anyform.
 */

defined('ABSPATH') || exit;

class AF_Email {

    /**
     * Send admin notification email.
     *
     * @param WP_Post $form The form post object.
     * @param array $submission_data The submitted form data.
     * @return bool|null True on success, false on failure, null if disabled.
     */
    public static function send_admin_notification($form, $submission_data) {
        if (!get_option('af_email_admin_enabled')) {
            return null;
        }

        $to = get_option('af_email_admin_to');
        if (empty($to)) {
            $to = get_option('admin_email');
        }

        $subject = self::parse_template(
            get_option('af_email_admin_subject', 'New form submission: {form_name}'),
            $form,
            $submission_data
        );

        $body = self::format_submission_email($form, $submission_data);

        return self::send($to, $subject, $body);
    }

    /**
     * Send auto-reply to form submitter.
     *
     * @param WP_Post $form The form post object.
     * @param array $submission_data The submitted form data.
     * @return bool|null True on success, false on failure, null if disabled/no email.
     */
    public static function send_auto_reply($form, $submission_data) {
        if (!get_option('af_email_reply_enabled')) {
            return null;
        }

        $email_field = get_option('af_email_reply_field', 'email');
        $to = $submission_data[$email_field] ?? null;

        if (empty($to) || !is_email($to)) {
            return null;
        }

        $subject = self::parse_template(
            get_option('af_email_reply_subject', 'Thank you for contacting us'),
            $form,
            $submission_data
        );

        $body = self::parse_template(
            get_option('af_email_reply_body', 'We received your message and will respond soon.'),
            $form,
            $submission_data
        );

        return self::send($to, $subject, $body);
    }

    /**
     * Send a test email using configured API method.
     *
     * @param string $to Recipient email.
     * @param string $subject Email subject.
     * @param string $body Email body (HTML).
     * @return bool True on success, false on failure.
     */
    public static function send_test($to, $subject, $body) {
        return self::send_api($to, $subject, $body);
    }

    /**
     * Send email using configured method.
     *
     * @param string $to Recipient email(s).
     * @param string $subject Email subject.
     * @param string $body Email body (HTML).
     * @return bool True on success, false on failure.
     */
    private static function send($to, $subject, $body) {
        $method = get_option('af_email_method', 'wp_mail');

        if ($method === 'api') {
            return self::send_api($to, $subject, $body);
        }

        return self::send_wp_mail($to, $subject, $body);
    }

    /**
     * Send email via WordPress wp_mail().
     */
    private static function send_wp_mail($to, $subject, $body) {
        $from_name = get_option('af_email_admin_from_name');
        if (empty($from_name)) {
            $from_name = get_bloginfo('name');
        }

        $from_email = get_option('af_email_admin_from_email');
        if (empty($from_email)) {
            $from_email = get_option('admin_email');
        }

        $headers = [
            'Content-Type: text/html; charset=UTF-8',
            "From: {$from_name} <{$from_email}>",
        ];

        $result = wp_mail($to, $subject, $body, $headers);

        if (!$result) {
            error_log('Anyform: wp_mail() failed to send email to ' . $to);
        }

        return $result;
    }

    /**
     * Send email via external API.
     */
    private static function send_api($to, $subject, $body) {
        $provider = get_option('af_email_api_provider', 'sendgrid');
        $api_key = get_option('af_email_api_key');

        if (empty($api_key)) {
            return false;
        }

        $from_name = get_option('af_email_admin_from_name', get_bloginfo('name'));
        $from_email = get_option('af_email_admin_from_email', get_option('admin_email'));

        // TODO: Per-form extension point

        switch ($provider) {
            case 'sendgrid':
                return self::send_sendgrid($to, $subject, $body, $api_key, $from_name, $from_email);
            case 'mailgun':
                return self::send_mailgun($to, $subject, $body, $api_key, $from_name, $from_email);
            case 'custom':
                return self::send_custom_api($to, $subject, $body, $api_key, $from_name, $from_email);
            default:
                return false;
        }
    }

    /**
     * Send via SendGrid API.
     */
    private static function send_sendgrid($to, $subject, $body, $api_key, $from_name, $from_email) {
        $to_addresses = array_map('trim', explode(',', $to));
        $personalizations = [];
        foreach ($to_addresses as $email) {
            if (is_email($email)) {
                $personalizations[] = ['email' => $email];
            }
        }

        $payload = [
            'personalizations' => [['to' => $personalizations]],
            'from' => [
                'email' => $from_email,
                'name' => $from_name,
            ],
            'subject' => $subject,
            'content' => [
                ['type' => 'text/html', 'value' => $body],
            ],
        ];

        $response = wp_remote_post('https://api.sendgrid.com/v3/mail/send', [
            'headers' => [
                'Authorization' => 'Bearer ' . $api_key,
                'Content-Type' => 'application/json',
            ],
            'body' => wp_json_encode($payload),
            'timeout' => 15,
        ]);

        if (is_wp_error($response)) {
            error_log('Anyform: SendGrid error - ' . $response->get_error_message());
            return false;
        }

        $code = wp_remote_retrieve_response_code($response);
        if ($code >= 400) {
            error_log('Anyform: SendGrid failed with code ' . $code . ' - ' . wp_remote_retrieve_body($response));
            return false;
        }

        return $code >= 200 && $code < 300;
    }

    /**
     * Send via Mailgun API.
     */
    private static function send_mailgun($to, $subject, $body, $api_key, $from_name, $from_email) {
        // Mailgun API key format: key-xxxxxxx or domain is in the key
        // For simplicity, assume US region. Domain should be configured separately.
        // This is a basic implementation - production would need domain config.

        $domain = get_option('af_email_api_endpoint'); // Reuse endpoint field for domain
        if (empty($domain)) {
            $domain = 'mg.' . parse_url(home_url(), PHP_URL_HOST);
        }

        $response = wp_remote_post("https://api.mailgun.net/v3/{$domain}/messages", [
            'headers' => [
                'Authorization' => 'Basic ' . base64_encode('api:' . $api_key),
            ],
            'body' => [
                'from' => "{$from_name} <{$from_email}>",
                'to' => $to,
                'subject' => $subject,
                'html' => $body,
            ],
            'timeout' => 15,
        ]);

        if (is_wp_error($response)) {
            error_log('Anyform: Mailgun error - ' . $response->get_error_message());
            return false;
        }

        $code = wp_remote_retrieve_response_code($response);
        if ($code >= 400) {
            error_log('Anyform: Mailgun failed with code ' . $code . ' - ' . wp_remote_retrieve_body($response));
            return false;
        }

        return $code >= 200 && $code < 300;
    }

    /**
     * Send via custom API endpoint.
     */
    private static function send_custom_api($to, $subject, $body, $api_key, $from_name, $from_email) {
        $endpoint = get_option('af_email_api_endpoint');
        if (empty($endpoint)) {
            return false;
        }

        // Generic JSON payload - adjust based on your API requirements
        $payload = [
            'to' => $to,
            'from' => [
                'email' => $from_email,
                'name' => $from_name,
            ],
            'subject' => $subject,
            'body' => $body,
            'html' => true,
        ];

        $response = wp_remote_post($endpoint, [
            'headers' => [
                'Authorization' => 'Bearer ' . $api_key,
                'Content-Type' => 'application/json',
            ],
            'body' => wp_json_encode($payload),
            'timeout' => 15,
        ]);

        if (is_wp_error($response)) {
            error_log('Anyform: Custom API error - ' . $response->get_error_message());
            return false;
        }

        $code = wp_remote_retrieve_response_code($response);
        if ($code >= 400) {
            error_log('Anyform: Custom API failed with code ' . $code . ' - ' . wp_remote_retrieve_body($response));
            return false;
        }

        return $code >= 200 && $code < 300;
    }

    /**
     * Parse template variables.
     */
    private static function parse_template($template, $form, $submission_data) {
        $replacements = [
            '{form_name}' => $form->post_title ?? 'Form',
            '{form_slug}' => $form->post_name ?? '',
            '{site_name}' => get_bloginfo('name'),
            '{site_url}' => home_url(),
            '{date}' => current_time('F j, Y'),
            '{time}' => current_time('g:i a'),
        ];

        // Add form field values
        foreach ($submission_data as $key => $value) {
            if (is_string($value)) {
                $replacements['{' . $key . '}'] = esc_html($value);
            }
        }

        return str_replace(array_keys($replacements), array_values($replacements), $template);
    }

    /**
     * Format submission data as HTML email.
     */
    private static function format_submission_email($form, $submission_data) {
        $form_name = esc_html($form->post_title ?? 'Form');
        $date = current_time('F j, Y \a\t g:i a');

        $html = "
        <div style=\"font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 600px; margin: 0 auto;\">
            <h2 style=\"color: #333; border-bottom: 2px solid #0073aa; padding-bottom: 10px;\">
                New Submission: {$form_name}
            </h2>
            <p style=\"color: #666; font-size: 14px;\">Received on {$date}</p>
            <table style=\"width: 100%; border-collapse: collapse; margin-top: 20px;\">
        ";

        foreach ($submission_data as $field => $value) {
            // Skip internal fields
            if (strpos($field, '_') === 0) {
                continue;
            }

            $label = ucwords(str_replace(['_', '-'], ' ', $field));
            $display_value = is_array($value) ? implode(', ', $value) : esc_html($value);

            $html .= "
                <tr>
                    <td style=\"padding: 12px; border-bottom: 1px solid #eee; font-weight: 600; width: 30%; vertical-align: top; color: #333;\">
                        {$label}
                    </td>
                    <td style=\"padding: 12px; border-bottom: 1px solid #eee; color: #555;\">
                        {$display_value}
                    </td>
                </tr>
            ";
        }

        $html .= "
            </table>
            <p style=\"margin-top: 30px; padding-top: 20px; border-top: 1px solid #eee; color: #888; font-size: 12px;\">
                This email was sent from <a href=\"" . home_url() . "\">" . get_bloginfo('name') . "</a>
            </p>
        </div>
        ";

        return $html;
    }
}
