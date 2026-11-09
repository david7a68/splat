---
splat_schema: 2
splat_id: 2108cd7b-cda7-54b0-94f1-220ff3ac75cc
splat_kind: spell
splat_key: dnd2024:spells/healing_word
splat_name: Healing Word
splat_origin: dnd2024:reference/spell/healing_word
splat_sources:
- SRD 5.2.1, pp. 139, https://www.dndbeyond.com/srd
---

# Healing Word


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes | Bard, Cleric, Druid |
| Casting Time | Bonus Action |
| Range | 60 feet |
| Components | V |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 139 -->

Level 1 Abjuration (Bard, Cleric, Druid)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous

A creature of your choice that you can see within range regains Hit Points equal to 2d4 plus your spellcasting ability modifier.

Using a Higher-Level Spell Slot. The healing increases by 2d4 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/139|page 139]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/healing_word ` |
| ` implementation ` | ` dnd2024:spell_healing_word ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
dice:
  count: 2
  sides: 4
  add:
    binding: spellcasting_modifier
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` healing_dice ` | ` 2 ` |
| ` sides ` | ` 4 ` |
