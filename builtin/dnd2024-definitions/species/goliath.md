---
splat_schema: 2
splat_id: cc9295b7-2cc8-5569-94a7-85b4c7002d17
splat_kind: rule_group
splat_key: dnd2024:species/goliath
splat_name: Goliath
splat_origin: dnd2024:reference/pages/085
splat_sources:
- SRD 5.2.1, pp. 85, 86, https://www.dndbeyond.com/srd
---

# Goliath

## Source text

<!-- SRD page 85 -->

Creature Type: Humanoid\
Size: Medium (about 7–8 feet tall)\
Speed: 35 feet

As a Goliath, you have these special traits.

Giant Ancestry. You are descended from Giants. Choose one of the following benefits—a supernatural boon from your ancestry; you can use the chosen benefit a number of times equal to your Proficiency Bonus, and you regain all expended uses when you finish a Long Rest:

Cloud’s Jaunt (Cloud Giant). As a Bonus Action, you magically teleport up to 30 feet to an unoccupied space you can see.

Fire’s Burn (Fire Giant). When you hit a target with an attack roll and deal damage to it, you can also deal 1d10 Fire damage to that target.

Frost’s Chill (Frost Giant). When you hit a target with an attack roll and deal damage to it, you can also deal 1d6 Cold damage to that target and reduce its Speed by 10 feet until the start of your next turn.

Hill’s Tumble (Hill Giant). When you hit a Large or smaller creature with an attack roll and deal damage to it, you can give that target the Prone condition.

Stone’s Endurance (Stone Giant). When you take damage, you can take a Reaction to roll 1d12. Add your Constitution modifier to the number rolled and reduce the damage by that total.

<!-- SRD page 86 -->

Storm’s Thunder (Storm Giant). When you take damage from a creature within 60 feet of you, you can take a Reaction to deal 1d8 Thunder damage to that creature.

Large Form. Starting at character level 5, you can change your size to Large as a Bonus Action if you’re in a big enough space. This transformation lasts for 10 minutes or until you end it (no action required). For that duration, you have Advantage on Strength checks, and your Speed increases by 10 feet. Once you use this trait, you can’t use it again until you finish a Long Rest.

Powerful Build. You have Advantage on any ability check you make to end the Grappled condition. You also count as one size larger when determining your carrying capacity.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/goliath ` |
| ` implementation ` | ` dnd2024:species_goliath ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` size ` | ` medium ` |
| ` speed_feet ` | ` 35 ` |

#### Height feet approximate

<!-- splat:values path=splat_rule/parameters/height_feet_approximate -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 7 ` |
| ` maximum ` | ` 8 ` |

#### Giant ancestry

<!-- splat:yaml path=splat_rule/parameters/giant_ancestry -->

```yaml
choice:
  count: 1
  options:
  - id: clouds_jaunt
    action: bonus_action
    teleport:
      maximum_feet: 30
      destination_visible: true
      destination_unoccupied: true
      magical: true
  - id: fires_burn
    trigger: attack_roll_hit_and_damage
    extra_damage:
      roll:
        dice:
          count: 1
          sides: 10
      type: fire
  - id: frosts_chill
    trigger: attack_roll_hit_and_damage
    extra_damage:
      roll:
        dice:
          count: 1
          sides: 6
      type: cold
    target_speed_reduction_feet: 10
    until: source.next_turn_start
  - id: hills_tumble
    trigger: attack_roll_hit_and_damage
    target_type: creature
    maximum_size: large
    condition: prone
  - id: stones_endurance
    trigger: damage.taken
    action: reaction
    damage_reduction:
      op: add
      args:
      - dice:
          count: 1
          sides: 12
      - stat: abilities.constitution.modifier
  - id: storms_thunder
    trigger: damage.taken_from_creature
    source_maximum_distance_feet: 60
    action: reaction
    damage_to_source:
      roll:
        dice:
          count: 1
          sides: 8
      type: thunder
uses:
  maximum:
    stat: proficiency_bonus
  recover:
    event: rest.long_rest_completed
    amount: all
```

#### Large form

<!-- splat:yaml path=splat_rule/parameters/large_form -->

```yaml
minimum_character_level: 5
action: bonus_action
requires_space_for_size: large
size: large
duration:
  minutes: 10
end_action: none
strength_check_advantage: true
speed_bonus_feet: 10
uses:
  maximum: 1
  recover:
    event: rest.long_rest_completed
    amount: all
```

#### Powerful build

<!-- splat:values path=splat_rule/parameters/powerful_build -->

| Field | Value |
| --- | --- |
| ` advantage_on_checks_to_end_condition ` | ` grappled ` |
| ` carrying_capacity_size_categories_bonus ` | ` 1 ` |
