---
splat_schema: 2
splat_id: f3af9c26-a19d-59e7-ac2c-593ee01d30b0
splat_kind: rule
splat_key: dnd2024:rules/weapons_vex
splat_name: Vex
splat_origin: dnd2024:reference/pages/090
splat_sources:
- SRD 5.2.1, pp. 90, https://www.dndbeyond.com/srd
---

# Vex

## Source text

<!-- SRD page 90 -->

If you hit a creature with this weapon and deal damage to the creature, you have Advantage on your next attack roll against that creature before the end of your next turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_vex ` |
| ` implementation ` | ` dnd2024:rule_weapons_vex ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` weapon_hits_and_damages_creature ` |

#### Effect

<!-- splat:values path=splat_rule/parameters/effect -->

| Field | Value |
| --- | --- |
| ` attacker_next_attack_against_same_creature ` | ` advantage ` |
| ` expires ` | ` end_of_attacker_next_turn ` |

#### Interleaved weapon table

<!-- splat:yaml path=splat_rule/parameters/interleaved_weapon_table -->

```yaml
entries:
- key: dnd2024:equipment/club
- key: dnd2024:equipment/dagger
- key: dnd2024:equipment/greatclub
- key: dnd2024:equipment/handaxe
- key: dnd2024:equipment/javelin
- key: dnd2024:equipment/light_hammer
- key: dnd2024:equipment/mace
- key: dnd2024:equipment/quarterstaff
- key: dnd2024:equipment/sickle
- key: dnd2024:equipment/spear
- key: dnd2024:equipment/dart
- key: dnd2024:equipment/light_crossbow
- key: dnd2024:equipment/shortbow
- key: dnd2024:equipment/sling
- key: dnd2024:equipment/battleaxe
- key: dnd2024:equipment/flail
- key: dnd2024:equipment/glaive
- key: dnd2024:equipment/greataxe
- key: dnd2024:equipment/greatsword
- key: dnd2024:equipment/halberd
- key: dnd2024:equipment/lance
- key: dnd2024:equipment/longsword
- key: dnd2024:equipment/maul
- key: dnd2024:equipment/morningstar
- key: dnd2024:equipment/pike
- key: dnd2024:equipment/rapier
- key: dnd2024:equipment/scimitar
- key: dnd2024:equipment/shortsword
- key: dnd2024:equipment/trident
- key: dnd2024:equipment/warhammer
- key: dnd2024:equipment/war_pick
- key: dnd2024:equipment/whip
- key: dnd2024:equipment/blowgun
- key: dnd2024:equipment/hand_crossbow
- key: dnd2024:equipment/heavy_crossbow
- key: dnd2024:equipment/longbow
- key: dnd2024:equipment/musket
- key: dnd2024:equipment/pistol
columns:
- category
- damage
- damage_type
- properties
- mastery
- weight
- cost
```
