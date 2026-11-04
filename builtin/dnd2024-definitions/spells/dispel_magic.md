---
splat_schema: 2
splat_id: 0091936d-2449-5293-bc84-d13f07ab9c8b
splat_kind: spell
splat_key: dnd2024:spells/dispel_magic
splat_name: Dispel Magic
splat_origin: dnd2024:reference/spell/dispel_magic
splat_sources:
- SRD 5.2.1, pp. 124, https://www.dndbeyond.com/srd
---

# Dispel Magic


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Abjuration |
| Classes | Bard, Cleric, Druid, Paladin, Ranger, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 124 -->

Level 3 Abjuration (Bard, Cleric, Druid, Paladin, Ranger, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous

Choose one creature, object, or magical effect within range. Any ongoing spell of level 3 or lower on the target ends. For each ongoing spell of level 4 or higher on the target, make an ability check using your spellcasting ability (DC 10 plus that spell’s level). On a successful check, the spell ends.

Using a Higher-Level Spell Slot. You automatically end a spell on the target if the spell’s level is equal to or less than the level of the spell slot you use.

Original layout: [[../../srd-5.2.1/pages/124|page 124]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dispel_magic ` |
| ` implementation ` | ` dnd2024:spell_dispel_magic ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_object_or_magical_effect ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Ongoing spells

<!-- splat:yaml path=splat_rule/parameters/ongoing_spells -->

```yaml
each: true
automatic_end_max_level:
  binding: cast_level
higher_level:
  check_ability: spellcasting
  dc:
    op: add
    args:
    - 10
    - binding: affected_spell_level
  success: end_spell
```
