# Rust for Backend Engineers: a 30-day healthcare project course

Learn Rust by evolving one small, synthetic healthcare information service. This repository is the complete student course, reference implementation, and instructor pack; it is not a sequence of unrelated tutorials.

> **Scope and safety:** This is educational software using synthetic data. It is not a medical device, EHR, or production-ready system for protected health information. Do not enter real patient data or deploy it for clinical use.

## Start here

1. Read [Course guide](COURSE.md) and [Setup](docs/setup.md).
2. Follow [Day 01](days/day-01.md) through [Day 30](days/day-30.md) in order; budget about 60 minutes each.
3. Use the `project/final` service as the evolving project. Each lesson names a small change, explains where it belongs, and links to the reference.
4. Attempt quizzes and exercises before opening the separated [instructor answers](instructor-guide/answer-keys.md) and [reference solutions](instructor-guide/exercise-solutions.md).
5. Commit your own progress as you go; Git is a notebook here, not a curriculum topic.

## Navigation

| Resource | Purpose |
|---|---|
| [30-day map](COURSE.md) | Daily topics, time budgets, project checkpoints |
| [Learning objectives](docs/learning-objectives.md) | Observable course outcomes |
| [Architecture overview](docs/architecture.md) | How the service evolves and why |
| [Glossary](docs/glossary.md) | Rust and backend terms |
| [Setup and validation](docs/setup.md) | Toolchain, database, and commands |
| [Final reference service](project/final/README.md) | Run and test the complete example |
| [Milestones](project/milestones/README.md) | Working checkpoints and change history |
| [ADRs](architecture/README.md) | Concise architectural decisions |
| [Instructor guide](instructor-guide/README.md) | Teaching plan, misconceptions, answer keys |

## Course promise

Every day follows **Learn → Code → Test → Review → Architect**. The project begins with domain values and ends with a modest, observable, authenticated HTTP service backed by PostgreSQL. The final architecture is deliberately a modular monolith; the last lessons examine when a separate service is worth its operational cost.

The course targets stable Rust and edition 2024. The official stable manifest was checked on 2026-10-31 and reported Rust 1.99.1 (manifest dated 2026-10-15). The `stable` toolchain is selected so that updates remain explicit and current; see [toolchain notes](docs/setup.md).
