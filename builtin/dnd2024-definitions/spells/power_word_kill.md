---
splat_schema: 2
splat_id: 1642deb2-80be-57eb-b4e0-0867bc724528
splat_kind: spell
splat_key: dnd2024:spells/power_word_kill
splat_name: Power Word Kill
splat_origin: dnd2024:reference/spell/power_word_kill
splat_sources:
- SRD 5.2.1, pp. 154, https://www.dndbeyond.com/srd
---

# Power Word Kill


## Spell

| Field | Value |
| --- | --- |
| Level | 9 |
| School | Enchantment |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 154 -->

Level 9 Enchantment (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Component: V\
Duration: Instantaneous

You compel one creature you can see within range to die. If the target has 100 Hit Points or fewer, it dies. Otherwise, it takes 12d12 Psychic damage.

Original layout: [[../../srd-5.2.1/pages/154|page 154]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/power_word_kill ` |
| ` implementation ` | ` dnd2024:spell_power_word_kill ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |

#### Branches

<!-- splat:table
path: splat_rule/parameters/branches
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  When stat:
    path: when/stat
    type: symbol
  When comparison:
    path: when/comparison
    type: symbol
  When value:
    path: when/value
    type: integer
  Effect:
    path: effect
    type: symbol
    optional: true
  Damage dice:
    path: damage/dice
    type: string
    optional: true
  Damage type:
    path: damage/type
    type: symbol
    optional: true
-->

| Key | When stat | When comparison | When value | Effect | Damage dice | Damage type |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Current hit points ` | ` Lte ` | ` 100 ` | ` Die ` |  |  |
| ` row_2 ` | ` Current hit points ` | ` Gt ` | ` 100 ` |  | ` 12d12 ` | ` Psychic ` |
