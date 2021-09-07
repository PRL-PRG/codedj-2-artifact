# Model report for file:///tmp/top-repos-quality-repos-16_69c5c/1-4_presents.git HEAD 5407ba0197e9b4a77ab861496cbc2e20d166bac6

### Dump

```json
{'created_at': '2021-09-02 10:44:57',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.2 kB',
 'tags': [],
 'uuid': '4504ccd8-fc8c-47c1-a514-424a221ca5f6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-16_69c5c/1-4_presents.git 5407ba0197e9b4a77ab861496cbc2e20d166bac6

# javascript
24 rules, avg.len. 9.4
## train
PPCR: 0.892556
### report
macro
{'f1-score': 0.7418638590618336,
 'precision': 0.7363262563462561,
 'recall': 0.7494834015881092,
 'support': 26425}
micro
{'f1-score': 0.9630652790917692,
 'precision': 0.9630652790917692,
 'recall': 0.9630652790917692,
 'support': 26425}
weighted
{'f1-score': 0.96101652474379,
 'precision': 0.959906551608623,
 'recall': 0.9630652790917692,
 'support': 26425}
### report_full
macro
{'f1-score': 0.6296855858921778,
 'precision': 0.7363262563462561,
 'recall': 0.5653243988266566,
 'support': 29606}
micro
{'f1-score': 0.9083899983937462,
 'precision': 0.9630652790917692,
 'recall': 0.8595892724447747,
 'support': 29606}
weighted
{'f1-score': 0.8892628477181543,
 'precision': 0.9315405317979633,
 'recall': 0.8595892724447747,
 'support': 29606}
## test
PPCR: 0.849569
### report
macro
{'f1-score': 0.6506703561976023,
 'precision': 0.6237841814733025,
 'recall': 0.7064787170894056,
 'support': 8076}
micro
{'f1-score': 0.9650817236255572,
 'precision': 0.9650817236255572,
 'recall': 0.9650817236255572,
 'support': 8076}
weighted
{'f1-score': 0.9638271376015685,
 'precision': 0.9635953612404368,
 'recall': 0.9650817236255572,
 'support': 8076}
### report_full
macro
{'f1-score': 0.48544880754895586,
 'precision': 0.6237841814733025,
 'recall': 0.43244881978355376,
 'support': 9506}
micro
{'f1-score': 0.8865885564782163,
 'precision': 0.9650817236255572,
 'recall': 0.8199032190195666,
 'support': 9506}
weighted
{'f1-score': 0.8627772845638974,
 'precision': 0.9357091787395755,
 'recall': 0.8199032190195666,
 'support': 9506}
```

## javascript
### Summary
19 rules, avg.len. 8.9

| | |
|-|-|
|Min support|100|
|Max support|5203|
|Min confidence|0.9417293071746826|
|Max confidence|0.9993954300880432|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.reserved not in {)}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.989. Support: 4379.` |
| 2 | `  +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 2029.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.995. Support: 111.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1269.` |
| 5 | `  -1.label in {<space>}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 827.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 749.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 754.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 613.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 590.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ,}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = FunctionDeclaration<br>⇒ y = ⏎⏎⇥⁺<br>Confidence: 0.948. Support: 106.` |
| 11 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {KEY} and not in {STRING}<br>	∧ +1.reserved not in {;}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 192.` |
| 12 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.roles in {CALLEE}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 177.` |
| 13 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles in {CALLEE}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 266.` |
| 14 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles in {FUNCTION} and not in {CALLEE}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 4<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 100.` |
| 15 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -2.length ≥ 2<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 230.` |
| 16 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.length ≤ 2<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 826.` |
| 17 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 5203.` |
| 18 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≥ 26<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.954. Support: 186.` |
| 19 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≤ 26<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 132.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.894736842105264, "max_conf": 0.9993954300880432, "max_support": 5203, "min_conf": 0.9417293071746826, "min_support": 100, "num_rules": 19}}
```
</details>
