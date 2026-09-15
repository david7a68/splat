---
splat_schema: 1
splat_id: 7faf89d8-1116-5718-ba6b-098b7b1944e3
splat_kind: item
splat_key: dnd2024:items/staff_of_healing
splat_name: Staff of Healing
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_healing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245
  url: https://www.dndbeyond.com/srd
---

# Staff of Healing

## Source text

<!-- SRD page 245 -->

Staff, Rare (Requires Attunement by a Bard, Cleric, or\
Druid)\
This staff has 10 charges. While holding the staff,\
you can cast one of the spells on the following table\
from it, using your spellcasting ability modifier. The\
table indicates how many charges you must expend\
to cast the spell.\
Spell Charge Cost\
Cure Wounds 1 charge per spell level\
(maximum 4 for a level 4\
spell)\
Lesser Restoration 2\
Mass Cure Wounds 5\
Regaining Charges. The staff regains 1d6 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, the staff vanishes in a\
flash of light, lost forever.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Bard, Cleric, or Druid ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_healing ` |
| ` implementation ` | ` dnd2024:item_staff_of_healing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 10
initial: 10
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 6
    bonus: 4
```

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | cost_per_spell_level | minimum_spell_level | maximum_spell_level | ability_modifier | cost |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 1 ` | ` 4 ` | ` user_spellcasting_ability_modifier ` |  |
| ` 1 ` |  |  |  | ` user_spellcasting_ability_modifier ` | ` 2 ` |
| ` 2 ` |  |  |  | ` user_spellcasting_ability_modifier ` | ` 5 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lesser_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_cure_wounds ` |

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  dice: 1
  sides: 20
  bonus: 0
on_value: 1
effect: vanish_permanently
```

#### Attunement

<!-- splat:yaml path=splat_rule/parameters/attunement -->

```yaml
required: true
eligibility:
  class_any:
  - bard
  - cleric
  - druid
```
