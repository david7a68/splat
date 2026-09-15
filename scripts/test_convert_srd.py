"""Source-based regression checks for conservative conversion."""
import unittest
from pathlib import Path
from convert_srd import creature,load_note,Unsupported

class ConversionTests(unittest.TestCase):
    def source(self,name):return load_note(Path('builtin/srd-5.2.1/creature')/(name+'.md'))
    def test_awakened_tree_keeps_defenses_and_attack(self):
        data=creature(*self.source('awakened_tree'))
        self.assertEqual(data['splat_stats']['max_hp']['value'],59)
        self.assertEqual(data['splat_traits']['damage_affinities']['parameters'],{'resistances':['bludgeoning','piercing'],'vulnerabilities':['fire']})
        self.assertEqual(data['splat_actions']['slam']['rolls']['damage']['expression'],'3d6 + 4')
    def test_multiattack_and_two_attack_modes_are_structured(self):
        data=creature(*self.source('black_bear'))
        self.assertEqual(data['splat_actions']['multiattack']['steps'][0]['count'],2)
        data=creature(*self.source('ogre'))
        self.assertEqual(data['splat_actions']['javelin_ranged']['targets']['chosen']['range'],{'feet':30,'long':120})
    def test_riders_recharge_and_passive_traits_are_never_dropped(self):
        for name in ['hill_giant','warhorse_skeleton','ape','hydra']:
            with self.subTest(name=name),self.assertRaises(Unsupported):creature(*self.source(name))
    def test_new_trailing_rule_cannot_silently_pass(self):
        meta,body=self.source('bandit')
        body=body.replace('Original layout:','The target also has the Prone condition.\n\nOriginal layout:')
        with self.assertRaises(Unsupported):creature(meta,body)

if __name__=='__main__':unittest.main()
