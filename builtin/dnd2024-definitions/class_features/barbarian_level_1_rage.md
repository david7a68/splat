---
splat_schema: 1
splat_id: 30b96f68-8540-5b47-ad85-b7ff206689d4
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_1_rage
splat_name: 'Level 1: Rage'
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28, 29
  url: https://www.dndbeyond.com/srd
---

# Level 1: Rage

## Source text

<!-- SRD page 28 -->

You can imbue yourself with a primal power called Rage, a force that grants you extraordinary might and resilience. You can enter it as a Bonus Action if you aren’t wearing Heavy armor.

You can enter your Rage the number of times shown for your Barbarian level in the Rages column of the Barbarian Features table. You regain one expended use when you finish a Short Rest, and you regain all expended uses when you finish a Long Rest.

**Barbarian Features**

| Level | Proficiency Bonus | Class Features | Rages | Rage Damage | Weapon Mastery |
| --- | --- | --- | --- | --- | --- |
| 1 | +2 | Rage, Unarmored Defense, Weapon Mastery | 2 | +2 | 2 |
| 2 | +2 | Danger Sense, Reckless Attack | 2 | +2 | 2 |
| 3 | +2 | Barbarian Subclass, Primal Knowledge | 3 | +2 | 2 |
| 4 | +2 | Ability Score Improvement | 3 | +2 | 3 |
| 5 | +3 | Extra Attack, Fast Movement | 3 | +2 | 3 |
| 6 | +3 | Subclass feature | 4 | +2 | 3 |
| 7 | +3 | Feral Instinct, Instinctive Pounce | 4 | +2 | 3 |
| 8 | +3 | Ability Score Improvement | 4 | +2 | 3 |
| 9 | +4 | Brutal Strike | 4 | +3 | 3 |
| 10 | +4 | Subclass feature | 4 | +3 | 4 |
| 11 | +4 | Relentless Rage | 4 | +3 | 4 |
| 12 | +4 | Ability Score Improvement | 5 | +3 | 4 |
| 13 | +5 | Improved Brutal Strike | 5 | +3 | 4 |
| 14 | +5 | Subclass feature | 5 | +3 | 4 |
| 15 | +5 | Persistent Rage | 5 | +3 | 4 |
| 16 | +5 | Ability Score Improvement | 5 | +4 | 4 |
| 17 | +6 | Improved Brutal Strike | 6 | +4 | 4 |
| 18 | +6 | Indomitable Might | 6 | +4 | 4 |
| 19 | +6 | Epic Boon | 6 | +4 | 4 |
| 20 | +6 | Primal Champion | 6 | +4 | 4 |

<!-- SRD page 29 -->

While active, your Rage follows the rules below.

Damage Resistance. You have Resistance to Bludgeoning, Piercing, and Slashing damage.

Rage Damage. When you make an attack using Strength—with either a weapon or an Unarmed Strike—and deal damage to the target, you gain a bonus to the damage that increases as you gain levels as a Barbarian, as shown in the Rage Damage column of the Barbarian Features table.

Strength Advantage. You have Advantage on Strength checks and Strength saving throws.

No Concentration or Spells. You can’t maintain Concentration, and you can’t cast spells.

Duration. The Rage lasts until the end of your next turn, and it ends early if you don Heavy armor or have the Incapacitated condition. If your Rage is still active on your next turn, you can extend the Rage for another round by doing one of the following:

•  Make an attack roll against an enemy.

•  Force an enemy to make a saving throw.

•  Take a Bonus Action to extend your Rage.

Each time the Rage is extended, it lasts until the end of your next turn. You can maintain a Rage for up to 10 minutes.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_1_rage ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_1_rage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` cost ` | ` 1 ` |
| ` ends_on ` | ` ["actor.heavy_armor_donned", "actor.incapacitated"] ` |
| ` resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` advantage ` | ` ["strength.check", "strength.save"] ` |
| ` prohibits ` | ` ["concentration", "spellcasting"] ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` wearing_heavy_armor ` | ` false ` |

#### Resource

<!-- splat:values path=splat_rule/parameters/resource -->

| Field | Value |
| --- | --- |
| ` id ` | ` barbarian.rage ` |

##### Maximum

<!-- splat:yaml path=splat_rule/parameters/resource/maximum -->

```yaml
table:
  key: dnd2024:tables/barbarian_progression
row:
  var: actor.class_levels.barbarian
column: rages
```

##### Recovery

<!-- splat:table
path: splat_rule/parameters/resource/recovery
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Details:
    type: section
    optional: true
-->

| Key | Event | Details |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_completed ` | [Rules](#^splat-d8de689e-row_1 "section:row_1") |
| ` row_2 ` | ` rest.long_completed ` | [Rules](#^splat-d8de689e-row_2 "section:row_2") |

###### Row 1

^splat-d8de689e-row_1

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recovery/@row_1/amount -->

```yaml
1
```

###### Row 2

^splat-d8de689e-row_2

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recovery/@row_2/amount -->

```yaml
all
```

#### Duration

<!-- splat:yaml path=splat_rule/parameters/duration -->

```yaml
end: actor.next_turn.end
maximum:
  minutes: 10
```

#### Damage bonus

<!-- splat:yaml path=splat_rule/parameters/damage_bonus -->

```yaml
requires:
  attack_ability: strength
  attack_kind:
  - weapon
  - unarmed_strike
  damage_dealt: true
amount:
  table:
    key: dnd2024:tables/barbarian_progression
  row:
    var: actor.class_levels.barbarian
  column: rage_damage
```

#### Extend

<!-- splat:yaml path=splat_rule/parameters/extend -->

```yaml
requires:
  rage_active: true
  turn: actor.turn_after_activation
any_event:
- attack.roll_against_enemy
- enemy.saving_throw_forced
- bonus_action.rage_extend
end: actor.next_turn.end
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 1 ` |
