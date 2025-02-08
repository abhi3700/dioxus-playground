# Browser

- I found that Safari browser on iOS devices show different result than other chromium-based browsers.
- Example:

  <details>
  <summary>Safari browser</summary>

  ![](../img/safari_browser_look.png)

  </details>

  <details>
  <summary>Other chromium-based browsers</summary>

  ![](../img/chromium_browser_look.png)

  </details>

  So, looking at this anomaly, I found that the issue is with the `viewport` meta tag with Safari browser. I tried fixing with different iterations, but didn't work so far.

  > Nevermind, it's not a big issue.
