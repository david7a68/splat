---
splat_schema: 2
splat_id: 967410a1-be27-5ac3-b0f8-f61e07e362d8
splat_kind: item
splat_key: dnd2024:items/staff_of_the_woodlands
splat_name: Staff of the Woodlands
splat_origin: dnd2024:reference/magic_item/staff_of_the_woodlands
splat_sources:
- SRD 5.2.1, pp. 247, https://www.dndbeyond.com/srd
---

# Staff of the Woodlands


## Item

| Field | Value |
| --- | --- |
| Type | Staff |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 247 -->

Staff, Rare (Requires Attunement by a Druid)

This staff has 6 charges and can be wielded as a magic Quarterstaff that grants a +2 bonus to attack rolls and damage rolls made with it. While holding it, you have a +2 bonus to spell attack rolls.

Spells. While holding the staff, you can cast one of the spells on the following table from it, using your spell save DC. The table indicates how many charges you must expend to cast the spell.

| Spell | Charge Cost |
| --- | --- |
| Animal Friendship | 1 |
| Awaken | 5 |
| Barkskin | 2 |
| Locate Animals or Plants | 2 |
| Pass without Trace | 2 |
| Speak with Animals | 1 |
| Speak with Plants | 3 |
| Wall of Thorns | 6 |

Tree Form. You can take a Magic action to plant one end of the staff in earth in an unoccupied space and expend 1 charge to transform the staff into a healthy tree. The tree is 60 feet tall and has a 5-foot-diameter trunk, and its branches at the top spread out in a 20-foot radius. The tree appears ordinary but radiates a faint aura of Transmutation magic that can be discerned with the Detect Magic spell. While touching the tree and using a Magic action, you return the staff to its normal form. Any creature in the tree falls when the tree reverts to a staff.

Regaining Charges. The staff regains 1d6 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff loses its properties and becomes a nonmagical Quarterstaff.

Original layout: [[../../srd-5.2.1/pages/247|page 247]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_the_woodlands ` |
| ` implementation ` | ` dnd2024:item_staff_of_the_woodlands ` |

### Parameters

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 6
initial: 6
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 6
    bonus: 0
```

#### Weapon

<!-- splat:yaml path=splat_rule/parameters/weapon -->

```yaml
base:
  key: dnd2024:equipment/quarterstaff
magical: true
attack_bonus: 2
damage_bonus: 2
```

#### Held

<!-- splat:values path=splat_rule/parameters/held -->

| Field | Value |
| --- | --- |
| ` spell_attack_bonus ` | ` 2 ` |

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- animal_friendship
- awaken
- barkskin
- locate_animals_or_plants
- pass_without_trace
- speak_with_animals
- speak_with_plants
- wall_of_thorns
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
  Requires:
    path: requires
    type: symbol
-->

| Spell | Cost | Save dc | Requires |
| --- | --- | --- | --- |
| [Animal Friendship](<../spells/animal_friendship.md> "dnd2024:spells/animal_friendship") | ` 1 ` | ` User spell save dc ` | ` Held ` |
| [Awaken](<../spells/awaken.md> "dnd2024:spells/awaken") | ` 5 ` | ` User spell save dc ` | ` Held ` |
| [Barkskin](<../spells/barkskin.md> "dnd2024:spells/barkskin") | ` 2 ` | ` User spell save dc ` | ` Held ` |
| [Locate Animals or Plants](<../spells/locate_animals_or_plants.md> "dnd2024:spells/locate_animals_or_plants") | ` 2 ` | ` User spell save dc ` | ` Held ` |
| [Pass without Trace](<../spells/pass_without_trace.md> "dnd2024:spells/pass_without_trace") | ` 2 ` | ` User spell save dc ` | ` Held ` |
| [Speak with Animals](<../spells/speak_with_animals.md> "dnd2024:spells/speak_with_animals") | ` 1 ` | ` User spell save dc ` | ` Held ` |
| [Speak with Plants](<../spells/speak_with_plants.md> "dnd2024:spells/speak_with_plants") | ` 3 ` | ` User spell save dc ` | ` Held ` |
| [Wall of Thorns](<../spells/wall_of_thorns.md> "dnd2024:spells/wall_of_thorns") | ` 6 ` | ` User spell save dc ` | ` Held ` |

#### Tree form

<!-- splat:yaml path=splat_rule/parameters/tree_form -->

```yaml
action: magic
requires:
  plant_end_in: earth
  space: unoccupied
cost: 1
height_ft: 60
trunk_diameter_ft: 5
top_branch_radius_ft: 20
appearance: ordinary_healthy_tree
detect_magic:
  school: transmutation
  intensity: faint
revert:
  action: magic
  requires: touch_tree
  occupants: fall
```

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  dice: 1
  sides: 20
  bonus: 0
on_value: 1
effect: nonmagical_quarterstaff
```

#### Attunement

<!-- splat:yaml path=splat_rule/parameters/attunement -->

```yaml
required: true
eligibility:
  class_any:
  - druid
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> equipment:
>   attunement_requirement_text: a Druid
> ```

