---
splat_schema: 2
splat_id: 948ae061-ee5e-5a75-b00a-ecc98ed4cbfc
splat_kind: item
splat_key: dnd2024:items/staff_of_charming
splat_name: Staff of Charming
splat_origin: dnd2024:reference/magic_item/staff_of_charming
splat_sources:
- SRD 5.2.1, pp. 245, https://www.dndbeyond.com/srd
---

# Staff of Charming


## Item

| Field | Value |
| --- | --- |
| Type | Staff |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 245 -->

Staff, Rare (Requires Attunement by a Bard, Cleric, Druid, Sorcerer, Warlock, or Wizard)

This staff has 10 charges. While holding the staff, you can use any of its properties:

Cast Spell. You can expend 1 of the staff’s charges to cast Charm Person, Command, or Comprehend Languages from it using your spell save DC.

Reflect Enchantment. If you succeed on a saving throw against an Enchantment spell that targets only you, you can take a Reaction to expend 1 charge from the staff and turn the spell back on its caster as if you had cast the spell.

Resist Enchantment. If you fail a saving throw against an Enchantment spell that targets only you, you can turn your failed save into a successful one. You can’t use this property of the staff again until the next dawn.

Regaining Charges. The staff regains 1d8 + 2 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff crumbles to dust and is destroyed.

Original layout: [[../../srd-5.2.1/pages/245|page 245]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_charming ` |
| ` implementation ` | ` dnd2024:item_staff_of_charming ` |

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
    sides: 8
    bonus: 2
```

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- charm_person
- command
- comprehend_languages
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
-->

| Spell | Cost | Save dc |
| --- | --- | --- |
| [Charm Person](<../spells/charm_person.md> "dnd2024:spells/charm_person") | ` 1 ` | ` User spell save dc ` |
| [Command](<../spells/command.md> "dnd2024:spells/command") | ` 1 ` | ` User spell save dc ` |
| [Comprehend Languages](<../spells/comprehend_languages.md> "dnd2024:spells/comprehend_languages") | ` 1 ` | ` User spell save dc ` |

#### Reflect

<!-- splat:values path=splat_rule/parameters/reflect -->

| Field | Value |
| --- | --- |
| ` action ` | ` reaction ` |
| ` trigger ` | ` successful_save_against_enchantment_spell_targeting_only_user ` |
| ` cost ` | ` 1 ` |
| ` target ` | ` original_caster ` |
| ` cast_as ` | ` user ` |

#### Resist

<!-- splat:yaml path=splat_rule/parameters/resist -->

```yaml
trigger: failed_save_against_enchantment_spell_targeting_only_user
resource:
  uses: 1
  recovery: dawn
replace_result: success
action: none
```

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

