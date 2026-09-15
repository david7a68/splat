---
splat_schema: 1
splat_id: 9d7ca33f-dea6-55d9-bc68-5e8e1725f3cb
splat_kind: rule
splat_key: dnd2024:rules/traps_poisoned_needle
splat_name: Poisoned Needle
splat_origin:
  key: dnd2024:reference/pages/200
splat_sources:
- title: SRD 5.2.1
  locator: pp. 200, 201
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/traps_poisoned_needle
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 200
  - 201
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
  parent_name: Traps
  line_span:
    first: 10133
    count: 30
splat_clauses:
- key: source_rules
  text: '# Poisoned Needle Nuisance Trap (Levels 1–4) Trigger: A creature opens the trap’s lock improperly
    or fails to disarm the trap Duration: Instantaneous A poisoned needle is hidden in a lock. When a
    creature opens the lock with any object other than the proper key, the needle springs out and stabs
    the creature. The creature makes a DC 11 Constitution saving throw. On a failed save, the creature
    takes 5 (1d10) Poison damage and has the Poisoned condition for 1 hour. On a successful save, the
    creature takes half as much damage only. Avoid. The trap doesn’t trigger if the lock is opened using
    a Knock spell or similar magic. Detect and Disarm. As a Search action, a creature can examine the
    trapped lock and make a DC 15 Wisdom (Perception) check, detecting the needle on a successful check.
    Once the trap is detected, a character can take an action to try to disarm the trap, doing so with
    a successful DC 15 Dexterity (Sleight of Hand) check. On a failed check, the creature triggers the
    trap. At Higher Levels. You can scale the trap for higher levels by increasing the damage and the
    save DC, as shown in the following table. Levels Poison Damage Save DC 5–10 11 (2d10) 13 11–16 22
    (4d10) 15 17–20 55 (10d10) 17'
  sha256: e0e3558d73ce99911002e4d1a623e222b1a50ca9ef49caaee3c16fc02b0a7fdf
  status: needs_semantic_conversion
---

# Poisoned Needle

<!-- SRD page 200 -->

Nuisance Trap (Levels 1–4)\
Trigger: A creature opens the trap’s lock improperly\
or fails to disarm the trap\
Duration: Instantaneous\
A poisoned needle is hidden in a lock. When a crea-\
ture opens the lock with any object other than the\
proper key, the needle springs out and stabs the\
creature. The creature makes a DC 11 Constitution\
saving throw. On a failed save, the creature takes\
5 (1d10) Poison damage and has the Poisoned\

<!-- SRD page 201 -->

condition for 1 hour. On a successful save, the crea-\
ture takes half as much damage only.\
Avoid. The trap doesn’t trigger if the lock is\
opened using a Knock spell or similar magic.\
Detect and Disarm. As a Search action, a creature\
can examine the trapped lock and make a DC 15\
Wisdom (Perception) check, detecting the needle on\
a successful check. Once the trap is detected, a char-\
acter can take an action to try to disarm the trap,\
doing so with a successful DC 15 Dexterity (Sleight\
of Hand) check. On a failed check, the creature trig-\
gers the trap.\
At Higher Levels. You can scale the trap for higher\
levels by increasing the damage and the save DC, as\
shown in the following table.\
Levels Poison Damage Save DC\
5–10 11 (2d10) 13\
11–16 22 (4d10) 15\
17–20 55 (10d10) 17\
