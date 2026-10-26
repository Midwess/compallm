---
title: "Mailing: ADR"
available: false
---

# Mailing: ADR

Decision: Accepted. Implementation: Queued SMTP delivery and the Loco consumer adapter are implemented; the feature remains under development.

## Problem and decision

Email delivery must not create an unbounded number of tasks or let repeated OTP requests flood one recipient. Implement Core's `EmailService` in `libs/mailing`, using Lettre's async SMTP client and one DI-owned Loco worker.

Queue acceptance is asynchronous: a successful request means the email was admitted, not delivered. AuthService decides when to request mail and retains ownership of its verification challenge. SMTP failures after admission are logged without exposing credentials, recipients, message bodies, or OTPs.

While running, the service logs successful SMTP sends every minute, including the count since the previous report and the total since startup.

All limits belong to SMTP settings supplied to the service constructor. The defaults are:

## Delivery limits

| Limit | Behavior |
| --- | --- |
| Concurrent sends | Ten consumers independently await the shared mailbox and each perform one send at a time |
| Waiting jobs | At most 1024 queued emails, in addition to the ten in progress |
| Full queue | Wait asynchronously for capacity for at most 30 seconds, then return an admission error |
| SMTP attempt | Whole delivery attempt times out after 60 seconds |
| Recipient cooldown | Skip duplicate delivery for 30 seconds after successful SMTP acceptance |

Reserve normalized recipient addresses while waiting for admission, queued, or sending, so concurrent duplicates are suppressed before the first delivery. Duplicate OTP requests return throttling and no new challenge; the first challenge remains usable. Failed, cancelled, or timed-out admission releases its reservation. Failed SMTP attempts release the recipient without automatic retries.

## Responsibilities and tradeoffs

Core owns the email request and templates. Mailing owns queue admission, recipient throttling, worker consumption, SMTP configuration, and delivery. DI supplies the service; the application starts and stops it with its lifecycle. Loco's normal per-job asynchronous spawning is insufficient for these bounds, so one worker runs MailingService, which joins the configured number of asynchronous receive/send loops without spawning a task per consumer or email. DI constructs the worker with MailingService; the application starts it after context initialization. The adapter only delegates `perform` to the service; it owns no locks or task handles. MailingService coordinates its own consumers and shutdown.

The queue and cooldown state are process-local. Restarting can lose accepted mail and clears cooldowns; it is not durable delivery or a multi-instance rate limiter. SMTP acceptance does not guarantee inbox delivery, and a timeout can leave delivery uncertain. Queued OTPs retain their original expiry; waiting does not extend the challenge. Shutdown stops admission, allows a bounded drain, then cancels remaining work.

Use the existing local MailDev service for development. Production SMTP requires explicit configuration. The API reports unavailable/full queues as temporary failure and recipient throttling as HTTP 429. Do not return a new OTP challenge when admission fails or is skipped.
