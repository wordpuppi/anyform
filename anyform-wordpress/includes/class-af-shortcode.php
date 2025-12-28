<?php
/**
 * Shortcode handler for Anyform.
 */

defined('ABSPATH') || exit;

class AF_Shortcode {

    private $has_form = false;

    /**
     * Constructor.
     */
    public function __construct() {
        add_action('wp_footer', [$this, 'enqueue_assets']);
    }

    /**
     * Render shortcode.
     */
    public function render($atts) {
        $atts = shortcode_atts([
            'slug' => '',
            'action_url' => '',
            'class' => '',
            'id' => '',
        ], $atts, 'anyform');

        if (empty($atts['slug'])) {
            if (current_user_can('edit_posts')) {
                $this->has_form = true; // Enqueue CSS for error styling
                return '<div class="af-error af-admin-error">' . esc_html__('Anyform: slug attribute required', 'anyform') . '</div>';
            }
            return '';
        }

        // Check for success redirect
        if (isset($_GET['af_success']) && $_GET['af_success'] === '1' &&
            isset($_GET['af_form']) && $_GET['af_form'] === $atts['slug']) {

            $this->has_form = true; // Enqueue CSS
            $form = AF_Post_Type::get_by_slug($atts['slug']);
            $settings = $form ? get_post_meta($form->ID, 'af_form_settings', true) : [];
            $message = !empty($settings['success_message'])
                ? $settings['success_message']
                : esc_html__('Thank you for your submission!', 'anyform');

            return '<div class="af-success-message" role="alert" aria-live="assertive"><p>' . esc_html($message) . '</p></div>';
        }

        $form = AF_Post_Type::get_by_slug($atts['slug']);
        if (!$form || $form->post_status !== 'publish') {
            if (current_user_can('edit_posts')) {
                $this->has_form = true; // Enqueue CSS for error styling
                // translators: %s: form slug
                return '<div class="af-error af-admin-error">' . sprintf(esc_html__('Anyform: form not found: %s', 'anyform'), esc_html($atts['slug'])) . '</div>';
            }
            return '';
        }

        $json = AF_Post_Type::get_json($form);
        if (!$json || empty($json['steps'])) {
            if (current_user_can('edit_posts')) {
                $this->has_form = true; // Enqueue CSS for error styling
                // translators: %s: form slug
                return '<div class="af-error af-admin-error">' . sprintf(esc_html__('Anyform: invalid form JSON for: %s', 'anyform'), esc_html($atts['slug'])) . '</div>';
            }
            return '';
        }

        $this->has_form = true;

        // Determine action URL
        $settings = get_post_meta($form->ID, 'af_form_settings', true) ?: [];
        $action_url = $atts['action_url']
            ?: ($settings['action_url'] ?? '')
            ?: rest_url("anyform/v1/forms/{$atts['slug']}");

        return $this->render_html($json, $atts, $action_url);
    }

    /**
     * Render form HTML.
     */
    private function render_html($json, $atts, $action_url) {
        $slug = esc_attr($atts['slug']);
        $form_id = $atts['id'] ?: "af-form-{$slug}";
        $class = "af-form " . esc_attr($atts['class']);
        $steps = $json['steps'] ?? [];
        $is_multi_step = count($steps) > 1;

        ob_start();
        ?>
        <form data-af-form="<?php echo esc_attr($slug); ?>"
              id="<?php echo esc_attr($form_id); ?>"
              action="<?php echo esc_url($action_url); ?>"
              method="post"
              class="<?php echo esc_attr(trim($class)); ?>">

            <?php wp_nonce_field("af_submit_{$slug}", 'af_nonce'); ?>

            <!-- Honeypot field for spam protection -->
            <input type="text" name="af_hp_<?php echo esc_attr($slug); ?>"
                   style="opacity:0;position:absolute;top:0;left:0;height:0;width:0;z-index:-1;"
                   tabindex="-1" autocomplete="off" aria-hidden="true">

            <?php foreach ($steps as $index => $step): ?>
                <?php echo $this->render_step($step, $index, $index === 0); ?>
            <?php endforeach; ?>

            <div class="af-navigation">
                <?php if ($is_multi_step): ?>
                    <button type="button" class="af-prev" disabled>
                        <?php esc_html_e('Back', 'anyform'); ?>
                    </button>
                    <button type="button" class="af-next">
                        <?php esc_html_e('Next', 'anyform'); ?>
                    </button>
                <?php endif; ?>
                <button type="submit" class="af-submit" <?php echo $is_multi_step ? 'style="display:none"' : ''; ?>>
                    <?php echo esc_html($json['settings']['submit_label'] ?? esc_html__('Submit', 'anyform')); ?>
                </button>
            </div>
        </form>
        <?php
        return ob_get_clean();
    }

    /**
     * Render a step.
     */
    private function render_step($step, $index, $visible) {
        $condition = !empty($step['condition']) ? wp_json_encode($step['condition']) : '';

        ob_start();
        ?>
        <div class="af-step"
             data-af-step="<?php echo (int) $index; ?>"
             data-af-visible="<?php echo $visible ? 'true' : 'false'; ?>"
             <?php if ($condition): ?>data-af-condition='<?php echo esc_attr($condition); ?>'<?php endif; ?>>

            <?php if (!empty($step['name'])): ?>
                <h3 class="af-step-title"><?php echo esc_html($step['name']); ?></h3>
            <?php endif; ?>

            <?php if (!empty($step['description'])): ?>
                <p class="af-step-description"><?php echo esc_html($step['description']); ?></p>
            <?php endif; ?>

            <?php foreach ($step['fields'] ?? [] as $field): ?>
                <?php echo $this->render_field($field); ?>
            <?php endforeach; ?>
        </div>
        <?php
        return ob_get_clean();
    }

    /**
     * Render a field.
     */
    private function render_field($field) {
        $name = esc_attr($field['name'] ?? '');
        $label = esc_html($field['label'] ?? $name);
        $type = $field['field_type'] ?? 'text';
        $required = !empty($field['validation']['required']);
        $placeholder = esc_attr($field['placeholder'] ?? '');
        $validation = wp_json_encode($field['validation'] ?? []);
        $condition = !empty($field['condition']) ? wp_json_encode($field['condition']) : '';

        // Build aria-describedby IDs
        $describedby_ids = [];
        if (!empty($field['help_text'])) {
            $describedby_ids[] = "af-help-{$name}";
        }
        $describedby_ids[] = "af-error-{$name}";
        $describedby = implode(' ', $describedby_ids);

        ob_start();
        ?>
        <div class="af-field"
             data-af-field="<?php echo esc_attr($name); ?>"
             data-af-visible="true"
             data-af-validation='<?php echo esc_attr($validation); ?>'
             <?php if ($condition): ?>data-af-condition='<?php echo esc_attr($condition); ?>'<?php endif; ?>>

            <label for="<?php echo esc_attr($name); ?>">
                <?php echo esc_html($label); ?>
                <?php if ($required): ?>
                    <span class="af-required" aria-hidden="true">*</span>
                <?php endif; ?>
            </label>

            <?php echo $this->render_input($field, $name, $placeholder, $required, $describedby); ?>

            <div class="af-error-message" id="af-error-<?php echo esc_attr($name); ?>" role="alert" aria-live="polite"></div>

            <?php if (!empty($field['help_text'])): ?>
                <div class="af-help-text" id="af-help-<?php echo esc_attr($name); ?>"><?php echo esc_html($field['help_text']); ?></div>
            <?php endif; ?>
        </div>
        <?php
        return ob_get_clean();
    }

    /**
     * Render input element.
     */
    private function render_input($field, $name, $placeholder, $required, $describedby = '') {
        $type = $field['field_type'] ?? 'text';
        $req = $required ? 'required' : '';
        $aria_req = $required ? 'aria-required="true"' : '';
        $aria_desc = $describedby ? "aria-describedby=\"{$describedby}\"" : '';
        $default = esc_attr($field['default_value'] ?? '');

        switch ($type) {
            case 'textarea':
                $rows = $field['ui_options']['rows'] ?? 4;
                return "<textarea name=\"{$name}\" id=\"{$name}\" placeholder=\"{$placeholder}\" rows=\"{$rows}\" {$req} {$aria_req} {$aria_desc}>{$default}</textarea>";

            case 'select':
                $options = '';
                foreach ($field['options'] ?? [] as $opt) {
                    $val = esc_attr($opt['value'] ?? '');
                    $lbl = esc_html($opt['label'] ?? $val);
                    $sel = $val === $default ? 'selected' : '';
                    $options .= "<option value=\"{$val}\" {$sel}>{$lbl}</option>";
                }
                return "<select name=\"{$name}\" id=\"{$name}\" {$req} {$aria_req} {$aria_desc}><option value=\"\">--</option>{$options}</select>";

            case 'radio':
                $inputs = '';
                foreach ($field['options'] ?? [] as $idx => $opt) {
                    $val = esc_attr($opt['value'] ?? '');
                    $lbl = esc_html($opt['label'] ?? $val);
                    $chk = $val === $default ? 'checked' : '';
                    $radio_id = "{$name}_{$idx}";
                    $inputs .= "<label class=\"af-radio\" for=\"{$radio_id}\"><input type=\"radio\" name=\"{$name}\" id=\"{$radio_id}\" value=\"{$val}\" {$chk}> {$lbl}</label>";
                }
                return "<div class=\"af-radio-group\" role=\"radiogroup\" {$aria_req} {$aria_desc}>{$inputs}</div>";

            case 'checkbox':
                $chk = $default ? 'checked' : '';
                return "<input type=\"checkbox\" name=\"{$name}\" id=\"{$name}\" value=\"1\" {$chk} {$aria_desc}>";

            default:
                $html_type = match ($type) {
                    'email' => 'email',
                    'url' => 'url',
                    'tel' => 'tel',
                    'number' => 'number',
                    'date' => 'date',
                    'time' => 'time',
                    'password' => 'password',
                    'hidden' => 'hidden',
                    default => 'text',
                };
                return "<input type=\"{$html_type}\" name=\"{$name}\" id=\"{$name}\" placeholder=\"{$placeholder}\" value=\"{$default}\" {$req} {$aria_req} {$aria_desc}>";
        }
    }

    /**
     * Enqueue CSS and JS if form was rendered.
     */
    public function enqueue_assets() {
        if (!$this->has_form) {
            return;
        }

        // CSS
        wp_enqueue_style('anyform-css', ANYFORM_URL . 'assets/css/anyform.css', [], ANYFORM_VERSION);

        // WASM client from CDN
        $js_url = 'https://cdn.jsdelivr.net/npm/@wordpuppi/anyform-wasm-js@' . ANYFORM_JS_VERSION . '/dist/index.js';

        // Inline init script (ES module)
        ?>
        <script type="module">
            import init, { hydrate_all } from '<?php echo esc_url($js_url); ?>';
            try {
                await init();
                hydrate_all();
            } catch (e) {
                console.warn('Anyform WASM init failed:', e);
            }
        </script>
        <?php
    }
}
