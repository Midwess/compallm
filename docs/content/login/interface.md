---
title: Login interface
available: false
---

# Login interface

Design status: Split layout, pill buttons, lightly rounded fields, golden-ratio proportions, doubling spacing scale, stable top and left alignment, six single-digit code fields, automatic code verification, and progressive resend controls accepted on 2026-10-27. Exact shades remain open to visual refinement. Implementation status: The Penpot login and signup screens use shared component instances for email, code entry, actions, recovery prompts, account prompts, and errors. Design foundations owns those bases and their states. Token-bound Auto Layout preserves the desktop composition. Mobile design is deferred. Real input behavior and API integration remain unimplemented; the existing portal signup page has not been replaced.

## Direction

Use a calm, dark-first interface with Cursor-inspired warm olive neutrals, deep blue-violet primary actions, and warm orange secondary accents. Purple replaces orange as the primary color; orange remains for secondary links and pixel details. Shared text-field surfaces follow the [portal control rule](../portal/controls.md): Linear background `#1A1A1B`, a 0.5-pixel `#48494D` border, and 8-pixel corners. Keep the authentication field heights and interaction states. Button radius is half the control height: 24 pixels for a 48-pixel button, explicitly applied to every corner. Remove step labels, technical form labels, angular actions, and unnecessary helper copy. Selected Pixelarticons and one small decorative spark provide nostalgic detail; there is no full retro theme or decorative looping animation.

The [Mux website](https://www.mux.com/) is a reference for deliberate graphic accents and fine structural borders. Adapt those principles to the dark olive design. Primary actions and focus use purple; secondary links and small brand details use orange. The coral error color keeps its separate meaning. Pixel details are decorative and must not become the only way to understand an action or status.

The desktop layout adapts [CodeRabbit's login](https://app.coderabbit.ai/login) with a roughly 62/38 split based on the golden ratio: a broad introduction beside a compact sign-in panel. Interpret the requested L rule as stable top and left alignment, informed by Linear: the brand anchors the upper-left and the form title, labels, and controls share a consistent left edge. Center the introduction within its column and center the form vertically in the narrower panel. Mobile prioritizes one compact form column and omits the secondary introduction. Remove the superseded login layouts from Penpot while retaining palette references. Do not copy provider capabilities, marketing claims, artwork, or legal links from references; Compallm retains its documented email-first flow.

### Spacing rule

Use the [shared spacing policy](../portal/spacing.md): halve the preceding spacing values with a 2 px minimum for nonzero spacing; zero remains zero. The current nominal doubling scale resolves to **1× = 4, 2× = 8, 4× = 16, 8× = 32, 16× = 64 pixels**. Golden-ratio proportions apply to the two-column split. Centering within available space is computed, not a new spacing token. Typography metrics, control dimensions, and corner radii are separate from spacing.

| Relationship | Spacing |
| --- | --- |
| Field label to input | 4 px |
| Adjacent OTP fields | 4 px |
| Input to Continue | 8 px |
| Heading to form or recipient description | 32 px |
| Recipient description to OTP fields | 4 px |
| Continue to signup prompt | 32 px |
| OTP fields to status, resend prompt, or resend button | 16 px |
| Recovery action to back action | 8 px |
| Input horizontal padding | 8 px |
| Mobile page padding | 16 px |
| Desktop brand inset | 32 px |
| Hero graphic to headline | 32 px |
| Hero headline to description | 16 px |

Bind Auto Layout gaps and padding to the named spacing tokens, rather than copying numeric positions. A heading/content stack uses 32 pixels. The email field group uses 4 pixels, its action group uses 8 pixels, OTP columns use 4 pixels, and error/recovery groups use 16 or 8 pixels as appropriate. Keep interactive rows at least 44 pixels high; visible text can sit inside those rows without introducing unrelated margins. Error and resend states follow the same scale instead of accumulating one-off offsets.

Penpot's **Design foundations** page is the single reference for colors, spacing, typography, component geometry, layout, accessibility, and interaction rules. Keep reusable component samples and spacing examples there; login pages contain only screen designs. Token-bound Auto Layout changes flow through to the linked layouts. They do not automatically update React or Storybook.

### Color palette

Use the dark-theme values published in [Cursor’s website CSS](https://cursor.com/): background `#14120B`, card `#1B1913`, elevated card `#201E18`, foreground `#EDECEC`, and orange accent `#F54E00`. Cursor mixes foreground opacity into its background for secondary text and borders; the opaque values below are derived equivalents for this design. Preserve our blue-violet primary actions instead of adopting Cursor’s neutral primary buttons. Use orange selectively for secondary links and pixel details.

| Role | Color | Source |
| --- | --- | --- |
| Page background | `#14120B` | Cursor dark background |
| Editorial panel | `#1B1913` | Cursor card |
| Input surface | `#1A1A1B` | Shared Linear field reference |
| Elevated surface | `#201E18` | Cursor elevated card |
| Quiet divider | `#292821` | Foreground at 10% on page |
| Input boundary | `#48494D` at 0.5 px | Shared Linear field reference |
| Primary text | `#EDECEC` | Cursor foreground |
| Supporting text | `#989895` | Derived secondary text |
| Primary action | `#5E5CE6` | Custom blue-violet |
| Primary hover | `#6461E8` | Custom blue-violet |
| Secondary links and pixel accents | `#F54E00` | Cursor orange accent |
| Secondary hover | `#FF6A2B` | Custom orange hover |
| Keyboard focus | `#9694FF` | Custom blue-violet |
| Text on the primary action | `#FFFFFF` | White |
| Error text and boundary | `#EFB0A4` | Retained warm coral |

Calculated contrast is 15.89:1 for primary text on the page and 6.08:1 for secondary text on cards. Primary-button text uses white on blue-violet at 5.06:1, with 4.75:1 on hover. Orange secondary text contrasts 5.33:1 on the page and 5.00:1 on cards. Text-field surfaces and boundaries now follow the shared Linear treatment; purple focus and coral error states retain their separate roles. Quiet dividers are decorative and must not serve as the only indication of an input boundary or state. These color checks do not establish accessibility of the eventual implementation.

Desktop places a brief product introduction beside a focused form. Mobile prioritizes the form and removes the secondary introduction. Keep fields visibly labeled, keyboard focus clear, and touch targets at least 44 pixels. Error text accompanies visual styling, so meaning never depends on color alone.

The login and signup designs use instances of the shared Text fields, Buttons, and Feedback components from Design foundations. Email uses the authentication field size; code digits use shared code-entry variants, including focus and error states. Preserve the current desktop composition, sample values, dimensions, and error messages while replacing independent shapes. Verify actual component links and propagation from a base, not just matching styles. This remains a canvas design change without API integration.

## Sign in with email

Start with a visibly labeled email field and a primary **Continue** button. Below Continue, show **New to Compallm? Sign up**, with Sign up styled as the secondary link to account creation. Keep the form otherwise minimal; omit a numbered step label and redundant descriptions. Signin and signup still have distinct endpoints and must not silently substitute for one another. Account-creation design is separate from this minimal sign-in composition.

After successful code admission, show **Check your email**, identify the submitted email, and display six separate single-character inputs. Each accepts one ASCII digit and has an accessible position label, such as “Digit 1 of 6,” within an “Email code” group. Typing advances focus; Backspace in an empty field moves to the previous field. Pasting a full code or accepting supported one-time-code autofill distributes its digits across the fields.

When all six digits are present, verify automatically. There is no Continue or Verify button on the code screen. Allow only one request in flight, announce **Checking code…**, and prevent duplicate submission of an unchanged code after a failure. Editing a digit or pasting a different complete code permits another attempt. Preserve the entered code and show an inline error on failure. On a temporary network failure, offer **Try again** for the verification request; do not require retyping or resend a code automatically.

Below the code fields, initially show **Didn't receive a code?**. Activating it reveals a conventional **Resend code** button; revealing the button does not send email. The resend button explicitly requests another code, displays its pending state, and obeys server throttling. After successful admission, return to code entry with cleared fields and focus on the first digit. A throttle or send failure retains the current challenge and shows recovery guidance. Keep **Use a different email** as a quiet back action. Do not promise delivery merely because the queue accepted a request.

Show the currently implemented ten-minute expiry. The older single-use and fifteen-minute product policy remains unresolved in [Email login](email.md); this design does not settle that discrepancy. Do not claim a code is single-use.

## States and recovery

### Signup design

Desktop signup reuses the selected split layout, Design foundations palette, pill button, input geometry, and token-bound Auto Layout. The dedicated Penpot **Signup** page contains four desktop screens: email entry, code verification, incorrect code, and email already registered. The email screen uses **Create your account**, an email field, **Continue**, and **Already have an account? Sign in**. Do not add a password, display-name field, workspace configuration, provider buttons, or unestablished legal links.

Continue requests a signup code, then opens **Verify your email** with the same six-field automatic-verification interaction and recovery pattern as signin. Successful verification creates the user and personal organization under the existing signup rules; subsequent onboarding is outside this screen design. Include the incorrect-code variant and the existing-email 409 state, which offers signin and does not pretend a code was sent. Preserve the login screens while adding signup variants. This is design work only; no API integration or mobile design is authorized by it.

- Invalid email: retain the input and explain how to correct it.
- Sending or verifying: show a working label and prevent duplicate submission.
- Incorrect or expired code: retain the email context, explain the failure, and offer another code without implying verification succeeded.
- Throttling: display the server's retry guidance when available; do not invent a universal client countdown.
- Queue or network failure: keep the form recoverable and allow another attempt.
- Successful verification: transition into the authenticated application; workspace onboarding is outside this screen design.

GitHub and GitLab remain planned and are omitted from this first email-focused concept. Their future visibility depends on enabled providers; this design does not authorize implementing OAuth. Legal and support destinations are not established, so the prototype must not invent policy links or support promises.

## Review

Current work is desktop-only. Use the user's revised email and code layouts as the visual basis for new variants, preserving their composition. Mobile design is deferred; deleted mobile screens must not be recreated. The incorrect-code variant retains the entered digits and adds coral boundaries and an inline text error, with recovery actions following the spacing scale.

Penpot shows static states and clickable transitions; it does not implement digit entry or validate real codes. A later authorized React implementation can provide keyboard behavior, paste/autofill, and automatic verification in Storybook. There is no automatic synchronization between the canvas and application code.
