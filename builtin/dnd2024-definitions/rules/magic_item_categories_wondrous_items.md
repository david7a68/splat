---
splat_schema: 1
splat_id: 28874c76-55dc-53ef-9452-3ac84e31d8b8
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_wondrous_items
splat_name: Wondrous Items
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_wondrous_items
  parameters:
    category: wondrous_item
    attunement_prerequisites:
      class: membership_in_required_class
      spellcaster:
        minimum_spells: 1
        allowed_sources:
        - traits
        - features
        excluded_sources:
        - magic_items
        - similar_external_sources
    wearable_size_adjustment:
      default: true
      gm_can_disable: true
      gm_fit_restrictions:
      - body_size
      - body_shape
    unusual_anatomy_fit:
      actor: gm
      scope: item_not_made_for_anatomy
      must_have_required_body_parts: true
    paired_items:
      default: both_worn
      gm_exception:
        scope: missing_body_part
        other_item_remains_on_person: true
  implementation: dnd2024:rule_magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:rule_magic_item_categories_wondrous_items
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - item_changes
  events: []
  capabilities:
  - magic_items
  - attunement
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 205
  engine_gaps: &id001
  - dnd2024:rule_magic_item_categories_wondrous_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 3cbd5825271b7fc43a5599d8eaca084f015bb2128935d657dd607116ba114289
  parent_name: Magic Item Categories
  line_span:
    first: 10525
    count: 32
splat_clauses:
- key: source_rules
  text: '# Wondrous Items Wondrous Items include wearable items such as boots, belts, capes, amulets,
    brooches, and circlets. Bags, carpets, figurines, horns, musical instruments, and more also fall into
    this category. Magic iteM Rules Rules for identifying, attuning to, and using magic items appear in
    “Equipment.” Additional rules are presented below. Attunement Prerequisites. If a magic item has a
    class prerequisite, a creature must be a member of that class to attune to the item. If a creature
    must be a spellcaster to attune to an item, the creature qualifies if it can cast at least one spell
    using its traits or features, not by using a magic item or the like. Items Made for Specific Creatures.
    Magic items that are meant to be worn tend to magically adjust themselves to the wearer. However,
    you can decide that a magic item doesn’t adjust its size to fit any wearer. For example, a particular
    armorer might make items usable only by folk who are sized and shaped like dwarves. Unusual Anatomy.
    Use your discretion to decide whether a creature can wear an item not made for its anatomy. A ring
    placed on a tentacle might work, but a yuan-ti with a snakelike tail instead of legs can’t wear magic
    boots. Paired Items. You can allow exceptions to the rule that paired items must both be worn. For
    example, a character with only one arm might be able to use a single Glove of Missile Snaring so long
    as the matching glove is on their person.'
  sha256: c870d134d81e1d235f2d8ff238a85ad4d26fb9460772649fd6a268a293f61bd0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wondrous Items

<!-- SRD page 205 -->

Wondrous Items include wearable items such\
as boots, belts, capes, amulets, brooches, and\
circlets. Bags, carpets, figurines, horns, musical\
instruments, and more also fall into this category.\
Magic iteM Rules\
Rules for identifying, attuning to, and using magic\
items appear in “Equipment.” Additional rules are\
presented below.\
Attunement Prerequisites. If a magic item has\
a class prerequisite, a creature must be a member of\
that class to attune to the item. If a creature must be\
a spellcaster to attune to an item, the creature qual-\
ifies if it can cast at least one spell using its traits or\
features, not by using a magic item or the like.\
Items Made for Specific Creatures. Magic\
items that are meant to be worn tend to magically\
adjust themselves to the wearer. However, you can\
decide that a magic item doesn’t adjust its size to fit\
any wearer. For example, a particular armorer might\
make items usable only by folk who are sized and\
shaped like dwarves.\
Unusual Anatomy. Use your discretion to decide\
whether a creature can wear an item not made for\
its anatomy. A ring placed on a tentacle might work,\
but a yuan-ti with a snakelike tail instead of legs can’t\
wear magic boots.\
Paired Items. You can allow exceptions to the\
rule that paired items must both be worn. For ex-\
ample, a character with only one arm might be able\
to use a single Glove of Missile Snaring so long as the\
matching glove is on their person.\
