---
splat_schema: 2
splat_id: f6d9bfb2-c33a-57d6-982b-847d019de796
splat_kind: rule
splat_key: dnd2024:glossary/attack_action
splat_name: Attack [Action]
splat_origin: dnd2024:reference/glossary/attack_action
splat_sources:
- SRD 5.2.1, pp. 177, https://www.dndbeyond.com/srd
---

# Attack [Action]

## Source text

<!-- SRD page 177 -->

When you take the Attack action, you can make one attack roll with a weapon or an Unarmed Strike.

Equipping and Unequipping Weapons. You can either equip or unequip one weapon when you make an attack as part of this action. You do so either before or after the attack. If you equip a weapon before an attack, you don’t need to use it for that attack. Equipping a weapon includes drawing it from a sheath or picking it up. Unequipping a weapon includes sheathing, stowing, or dropping it.

Moving between Attacks. If you move on your turn and have a feature, such as Extra Attack, that gives you more than one attack as part of the Attack action, you can use some or all of that movement to move between those attacks.

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/attack_action ` |
| ` implementation ` | ` dnd2024:glossary_attack_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` default_attacks ` | ` 1 ` |
| ` attack_options ` | ` ["weapon", "unarmed_strike"] ` |

#### Equipment interaction

<!-- splat:yaml path=splat_rule/parameters/equipment_interaction -->

```yaml
per_attack: 1
operation:
  chooser: subject
  options:
  - equip
  - unequip
  - none
timing:
  chooser: subject
  options:
  - before_attack
  - after_attack
equipped_weapon_must_be_used: false
equip_methods:
- draw
- pick_up
unequip_methods:
- sheathe
- stow
- drop
```

#### Multiple attacks

<!-- splat:values path=splat_rule/parameters/multiple_attacks -->

| Field | Value |
| --- | --- |
| ` requires ` | ` feature_granted_additional_attacks ` |
| ` movement_between_attacks ` | ` true ` |
| ` movement_spend ` | ` some_or_all_available_turn_movement ` |
