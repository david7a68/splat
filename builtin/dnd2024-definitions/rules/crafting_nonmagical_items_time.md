---
splat_schema: 1
splat_id: 97528665-617f-569a-90be-475da1c599f7
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items_time
splat_name: Time
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_nonmagical_items_time
  parameters:
    work_hours_per_day: 8
    base_days:
      op: ceil
      args:
      - op: divide
        args:
        - var: item.purchase_cost_gp
        - 10
    days_must_be_consecutive: false
    assistance:
      divide_time_by: worker_count
      default_assistants_max: 1
      higher_assistant_limit_authority: gm
  implementation: dnd2024:rules_crafting_nonmagical_items_time
splat_contracts:
- id: dnd2024:rules_crafting_nonmagical_items_time
  inputs:
    item:
      type: definition
      required: true
    workers:
      type: participant_list
      required: true
    assistant_limit:
      type: integer
      required: true
  outputs:
  - crafting_days
  events: []
  capabilities:
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rules_crafting_nonmagical_items_time
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 1078dc66b68dfb7bce2a9d0c91a18cb5dff9a394f071e21a152cc218ed3dd756
  parent_name: Crafting Nonmagical Items
  line_span:
    first: 8978
    count: 13
splat_clauses:
- key: source_rules
  text: '# Time To determine how many days (working 8 hours a day) it takes to make an item, divide its
    purchase cost in GP by 10 (round a fraction up to a day). For example, you need 5 days to make a Heavy
    Crossbow, which sells for 50 GP. If an item requires multiple days, the days needn’t be consecutive.
    Characters can combine their efforts to shorten the crafting time. Divide the time needed to create
    an item by the number of characters working on it. Normally, only one other character can assist you,
    but the GM might allow more assistants.'
  sha256: a867f9fc310b473ca5f5fd5f967b8ac0d0b980c5c805595c45a2b4d5292b7937
  status: contract_specified
splat_requires_mechanics: *id001
---

# Time

<!-- SRD page 103 -->

To determine how many days (working 8 hours a\
day) it takes to make an item, divide its purchase\
cost in GP by 10 (round a fraction up to a day). For\
example, you need 5 days to make a Heavy Cross-\
bow, which sells for 50 GP.\
If an item requires multiple days, the days needn’t\
be consecutive.\
Characters can combine their efforts to shorten\
the crafting time. Divide the time needed to create\
an item by the number of characters working on it.\
Normally, only one other character can assist you,\
but the GM might allow more assistants.\
