# getsecure

Utility for securing expiring links

## Usage

### Add to nginx vhost secure location section like this

```nginx
    location /secret_page.html {
     secure_link $arg_md5,$arg_expires;
     secure_link_md5 "$secure_link_expires$uri <put here secret string>";

     if ($secure_link = "") {
         return 403;
     }

     if ($secure_link = "0") {
         return 410;
     }
    }
```

### Get secure link

```bash
getsecure http://example.com/secret_page.html <put here secret string> --period=30
http://example.com/secret_page.html?md5=IQ6H6OQYGeUnlLEDgHfYNw&expires=1488221041
```

or

```bash
getsecure /secret_page.html <put here secret string> --period=30
/secret_page.html?md5=IQ6H6OQYGeUnlLEDgHfYNw&expires=1488221041
```

where period is days from now to future when link should work
