---
splat_schema: 1
splat_id: f4ab86fd-f6e9-5557-9649-abe86bb353b1
splat_kind: item
splat_key: dnd2024:items/staff_of_the_python
splat_name: Staff of the Python
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_the_python
splat_sources:
- title: SRD 5.2.1
  locator: pp. 247
  url: https://www.dndbeyond.com/srd
---

# Staff of the Python

## Source text

<!-- SRD page 247 -->

Staff, Uncommon (Requires Attunement)\
As a Magic action, you can throw this staff so that it\
lands in an unoccupied space within 10 feet of you,\
causing the staff to become a Giant Constrictor\
Snake in that space. The snake is under your con-\
trol and shares your Initiative count, taking its turn\
immediately after yours.\
On your turn, you can mentally command the\
snake (no action required) if it is within 60 feet of\
you and you don’t have the Incapacitated condition.\
You decide what action the snake takes and where it\
moves during its turn, or you can issue it a general\
command, such as to attack your enemies or guard\
a location. Absent commands from you, the snake\
defends itself.\
As a Bonus Action, you can command the snake\
to revert to staff form in its current space, and you\
can’t use the staff’s property again for 1 hour. If the\
snake is reduced to 0 Hit Points, it dies and reverts\
to its staff form; the staff then shatters and is de-\
stroyed. If the snake reverts to staff form before\
losing all its Hit Points, it regains all of them.\

Original layout: [[../../srd-5.2.1/pages/247|page 247]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_the_python ` |
| ` implementation ` | ` dnd2024:item_staff_of_the_python ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` snake_zero_hp ` | ` ["snake_dies", "revert_to_staff", "staff_shatters_and_destroyed"] ` |
| ` attunement_required ` | ` true ` |

#### Activate

<!-- splat:yaml path=splat_rule/parameters/activate -->

```yaml
action: magic
throw_range_ft: 10
unoccupied_space: true
transform:
  key: dnd2024:creatures/giant_constrictor_snake
```

#### Snake

<!-- splat:yaml path=splat_rule/parameters/snake -->

```yaml
controller: user
initiative: user_initiative
turn: immediately_after_user
command:
  action: none
  'on': user_turn
  mental: true
  range_ft: 60
  requires_user_not_incapacitated: true
  choices:
  - action_and_movement
  - general_command
no_commands: defend_self
```

#### Revert

<!-- splat:yaml path=splat_rule/parameters/revert -->

```yaml
action: bonus_action
command: true
location: snake_current_space
heal_snake_to_full: true
cooldown:
  amount: 1
  unit: hours
```
