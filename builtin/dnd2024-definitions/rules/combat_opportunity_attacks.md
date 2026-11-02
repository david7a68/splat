---
splat_schema: 1
splat_id: 2f3e8ffa-77e7-5fbc-8a94-bf06f8ee3523
splat_kind: rule
splat_key: dnd2024:rules/combat_opportunity_attacks
splat_name: Opportunity Attacks
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
---

# Opportunity Attacks

## Source text

<!-- SRD page 15 -->

Combatants watch for enemies to drop their guard. If you move heedlessly past your foes, you put yourself in danger by provoking an Opportunity Attack.

Avoiding Opportunity Attacks. You can avoid provoking an Opportunity Attack by taking the Disengage action. You also don’t provoke an Opportunity Attack when you Teleport or when you are moved without using your movement, action, Bonus Action, or Reaction. For example, you don’t provoke an Opportunity Attack if an explosion hurls you out of a foe’s reach or if you fall past an enemy.

Making an Opportunity Attack. You can make an Opportunity Attack when a creature that you can see leaves your reach. To make the attack, take a Reaction to make one melee attack with a weapon or an Unarmed Strike against that creature. The attack occurs right before it leaves your reach.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_opportunity_attacks ` |
| ` implementation ` | ` dnd2024:rule_combat_opportunity_attacks ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` reaction ` |
| ` attack_count ` | ` 1 ` |
| ` attack_options ` | ` ["melee_weapon", "unarmed_strike"] ` |
| ` attack_timing ` | ` immediately_before_leaves_reach ` |
| ` prevented_by ` | ` ["disengage", "teleportation", "movement_without_target_movement_action_bonus_action_or_reaction"] ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` creature.leaves_reach ` |
| ` target_visible ` | ` true ` |
