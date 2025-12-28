# Anyform WordPress Plugin Tests

This directory contains the PHPUnit tests for the Anyform WordPress plugin.

Tests are kept separate from the main plugin to keep the WordPress.org distribution clean.

## Setup

1. Install dependencies:
   ```bash
   composer install
   ```

2. Install WordPress test library:
   ```bash
   bash bin/install-wp-tests.sh wordpress_test root '' localhost latest
   ```

   Adjust database credentials as needed.

3. Run tests:
   ```bash
   ./vendor/bin/phpunit
   ```

## Structure

```
anyform-wordpress-tests/
├── bin/
│   └── install-wp-tests.sh    # WP test library installer
├── tests/
│   ├── bootstrap.php          # PHPUnit bootstrap
│   └── phpunit/               # Test files
├── composer.json
├── phpunit.xml.dist
└── README.md
```

## Notes

- Tests reference the plugin at `../anyform-wordpress/`
- The `ANYFORM_PLUGIN_DIR` constant in `phpunit.xml.dist` controls the plugin path
