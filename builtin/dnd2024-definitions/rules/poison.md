---
splat_schema: 2
splat_id: 213bef09-f991-5bb0-86db-d0aceb89e844
splat_kind: rule
splat_key: dnd2024:rules/poison
splat_name: Poison
splat_origin: dnd2024:reference/pages/197
splat_sources:
- SRD 5.2.1, pp. 197, https://www.dndbeyond.com/srd
---

# Poison

## Source text

<!-- SRD page 197 -->

Given their insidious and deadly nature, poisons are a favorite tool among assassins and evil creatures.

Poisons come in the following four types:

Contact. Contact poison can be smeared on an object and remains potent until it is touched or washed off. A creature that touches contact poison with exposed skin suffers its effects.

Ingested. A creature must swallow an entire dose of ingested poison to suffer its effects. The dose can be delivered in food or a liquid. You may decide that a partial dose has a reduced effect, such as allowing Advantage on the saving throw or dealing only half as much damage on a failed save.

Inhaled. Poisonous powders and gases take effect when inhaled. Blowing the powder or releasing the gas subjects creatures in a 5-foot Cube to its effect. The resulting cloud dissipates immediately afterward. Holding one’s breath is ineffective against inhaled poisons, as they affect nasal membranes, tear ducts, and other parts of the body.

Injury. Injury poison can be applied as a Bonus Action to a weapon, a piece of ammunition, or similar object. The poison remains potent until delivered through a wound or washed off. A creature that takes Piercing or Slashing damage from an object coated with the poison is exposed to its effects.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison ` |
| ` implementation ` | ` dnd2024:rule_poison ` |

### Parameters

#### Types

<!-- splat:yaml path=splat_rule/parameters/types -->

```yaml
contact:
  application: smear_on_object
  potent_until_any:
  - exposed_skin_touch
  - washed_off
  exposure: exposed_skin_touch
ingested:
  full_effect_requires: swallow_entire_dose
  delivery:
  - food
  - liquid
  partial_dose:
    actor: gm
    choices:
    - no_effect
    - saving_throw: advantage
    - failed_save_damage_multiplier: 0.5
inhaled:
  delivery:
  - blown_powder
  - released_gas
  area:
    shape: cube
    edge_feet: 5
  cloud_duration: immediately_dissipates_after_exposure
  holding_breath_prevents: false
injury:
  application_action: bonus_action
  apply_to:
  - weapon
  - ammunition_piece
  - similar_object
  potent_until_any:
  - delivered_through_wound
  - washed_off
  exposure_requires:
    coated_object_damage_type:
    - piercing
    - slashing
```
