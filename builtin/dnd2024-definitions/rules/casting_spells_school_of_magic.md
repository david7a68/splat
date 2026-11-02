---
splat_schema: 1
splat_id: 62f14d6b-ec6c-53bf-83be-deeb4deb3b49
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_school_of_magic
splat_name: School of Magic
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
---

# School of Magic

## Source text

<!-- SRD page 105 -->

Each spell belongs to a school of magic. The schools are listed in the Schools of Magic table. These categories help describe spells but have no rules of their own, although some other rules refer to them.

**Schools of Magic**

| School | Typical Effects |
| --- | --- |
| Abjuration | Prevents or reverses harmful effects |
| Conjuration | Transports creatures or objects |
| Divination | Reveals information |
| Enchantment | Influences minds |
| Evocation | Channels energy to create effects that are often destructive |
| Illusion | Deceives the mind or senses |
| Necromancy | Manipulates life and death |
| Transmutation | Transforms creatures or objects |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_school_of_magic ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_school_of_magic ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` classification_only ` | ` true ` |
| ` schools ` | ` ["abjuration", "conjuration", "divination", "enchantment", "evocation", "illusion", "necromancy", "transmutation"] ` |
| ` standalone_school_effects ` | ` false ` |
| ` other_rules_can_match_school ` | ` true ` |
