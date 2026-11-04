---
splat_schema: 2
splat_id: b6a26867-9df9-5f89-86a4-24f67cb6926e
splat_kind: item
splat_key: dnd2024:items/staff_of_swarming_insects
splat_name: Staff of Swarming Insects
splat_origin: dnd2024:reference/magic_item/staff_of_swarming_insects
splat_sources:
- SRD 5.2.1, pp. 246, https://www.dndbeyond.com/srd
---

# Staff of Swarming Insects


## Item

| Field | Value |
| --- | --- |
| Type | Staff |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 246 -->

Staff, Rare (Requires Attunement by a Bard, Cleric, Druid, Sorcerer, Warlock, or Wizard)

This staff has 10 charges.

Insect Cloud. While holding the staff, you can take a Magic action and expend 1 charge to cause a swarm of harmless flying insects to fill a 30-foot Emanation originating from you. The insects remain for 10 minutes, making the area Heavily Obscured for creatures other than you. A strong wind (like that created by Gust of Wind) disperses the swarm and ends the effect.

Spells. While holding the staff, you can cast one of the spells on the following table from it, using your spell save DC and spell attack modifier. The table indicates how many charges you must expend to cast the spell.

| Spell | Charge Cost |
| --- | --- |
| Giant Insect | 4 |
| Insect Plague | 5 |

Regaining Charges. The staff regains 1d6 + 4 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, a swarm of insects consumes and destroys the staff, then disperses.

Original layout: [[../../srd-5.2.1/pages/246|page 246]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_swarming_insects ` |
| ` implementation ` | ` dnd2024:item_staff_of_swarming_insects ` |

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

#### Insect cloud

<!-- splat:yaml path=splat_rule/parameters/insect_cloud -->

```yaml
action: magic
cost: 1
shape: emanation
radius_ft: 30
origin: user
duration:
  value: 10
  unit: minute
insects_harmless: true
area: heavily_obscured
vision_exception: user
end_on: strong_wind
```

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- giant_insect
- insect_plague
columns:
  Spell:
    path: spell
    type: reference
  Cost:
    path: cost
    type: integer
  Save dc:
    path: save_dc
    type: symbol
  Attack modifier:
    path: attack_modifier
    type: symbol
-->

| Spell | Cost | Save dc | Attack modifier |
| --- | --- | --- | --- |
| [Giant Insect](<../spells/giant_insect.md> "dnd2024:spells/giant_insect") | ` 4 ` | ` User spell save dc ` | ` User spell attack modifier ` |
| [Insect Plague](<../spells/insect_plague.md> "dnd2024:spells/insect_plague") | ` 5 ` | ` User spell save dc ` | ` User spell attack modifier ` |

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  dice: 1
  sides: 20
  bonus: 0
on_value: 1
effect: destroy_item
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
  - sorcerer
  - warlock
  - wizard
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> equipment:
>   attunement_requirement_text: a Bard, Cleric, Druid, Sorcerer, Warlock, or Wizard
> ```

