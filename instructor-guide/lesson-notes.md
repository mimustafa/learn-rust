# Instructor notes by phase

## Days 1–4: Build compiler trust

Show short examples and ask learners to predict ownership before running them. Do not rescue learners by adding `.clone()` to every borrow error: first ask who should own the data and whether the function needs a value or a reference.

## Days 5–10: Use types deliberately

Connect domain states to enums and absence to `Option`. Distinguish a trait's role from an interface-shaped habit. Lifetimes label relationships; they do not extend storage duration. Let elision work where possible.

## Days 11–18: Make boundaries testable

Discuss error mapping at boundaries and keeping internal database detail out of HTTP responses. Use the SQL parameter exercise to contrast bound values with string formatting. Keep live database work optional if shared classroom setup is unavailable.

## Days 19–26: Async and operations

Emphasize that futures are lazy state machines and `.await` yields rather than creating a thread. Ask what resources are bounded and what happens on timeout/cancellation. Separate liveness from readiness; avoid logging secrets or patient data.

## Days 27–30: Architecture and teach-back

Do not reward the number of patterns or services. Ask what requirement justifies each boundary and what failure mode a new service adds. End with a learner-owned implementation and a short explanation, not a certification.

The exact outcomes and misconceptions are listed in [common mistakes](common-mistakes.md) and [discussion topics](discussion-topics.md).
