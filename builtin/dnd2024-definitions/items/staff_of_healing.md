---
splat_schema: 2
splat_id: 7faf89d8-1116-5718-ba6b-098b7b1944e3
splat_kind: item
splat_key: dnd2024:items/staff_of_healing
splat_name: Staff of Healing
splat_origin: dnd2024:reference/magic_item/staff_of_healing
splat_sources:
- SRD 5.2.1, pp. 245, https://www.dndbeyond.com/srd
---

# Staff of Healing


## Item

| Field | Value |
| --- | --- |
| Type | Staff |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 245 -->

Staff, Rare (Requires Attunement by a Bard, Cleric, or Druid)

This staff has 10 charges. While holding the staff, you can cast one of the spells on the following table from it, using your spellcasting ability modifier. The table indicates how many charges you must expend to cast the spell.

| Spell | Charge Cost |
| --- | --- |
| Cure Wounds | 1 charge per spell level (maximum 4 for a level 4 spell) |
| Lesser Restoration | 2 |
| Mass Cure Wounds | 5 |

Regaining Charges. The staff regains 1d6 + 4 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff vanishes in a flash of light, lost forever.

Original layout: [[../../srd-5.2.1/pages/245|page 245]].





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

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- cure_wounds
- lesser_restoration
- mass_cure_wounds
columns:
  Spell:
    path: spell
    type: reference
  Cost per spell level:
    path: cost_per_spell_level
    type: integer
    optional: true
  Minimum spell level:
    path: minimum_spell_level
    type: integer
    optional: true
  Maximum spell level:
    path: maximum_spell_level
    type: integer
    optional: true
  Ability modifier:
    path: ability_modifier
    type: symbol
  Cost:
    path: cost
    type: integer
    optional: true
-->

| Spell | Cost per spell level | Minimum spell level | Maximum spell level | Ability modifier | Cost |
| --- | --- | --- | --- | --- | --- |
| [Cure Wounds](<../spells/cure_wounds.md> "dnd2024:spells/cure_wounds") | ` 1 ` | ` 1 ` | ` 4 ` | ` User spellcasting ability modifier ` |  |
| [Lesser Restoration](<../spells/lesser_restoration.md> "dnd2024:spells/lesser_restoration") |  |  |  | ` User spellcasting ability modifier ` | ` 2 ` |
| [Mass Cure Wounds](<../spells/mass_cure_wounds.md> "dnd2024:spells/mass_cure_wounds") |  |  |  | ` User spellcasting ability modifier ` | ` 5 ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> equipment:
>   attunement_requirement_text: a Bard, Cleric, or Druid
> ```

