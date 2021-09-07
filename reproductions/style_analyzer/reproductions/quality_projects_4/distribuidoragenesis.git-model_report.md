# Model report for file:///tmp/top-repos-quality-repos-2pdd6bsp/distribuidoragenesis.git HEAD ef55a3ba001fd57db80e68a9add969136e76ff95

### Dump

```json
{'created_at': '2021-09-02 17:17:16',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '22.5 kB',
 'tags': [],
 'uuid': '7410d55c-8b55-4ec1-96c7-3969ddc21a3f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2pdd6bsp/distribuidoragenesis.git ef55a3ba001fd57db80e68a9add969136e76ff95

# javascript
51 rules, avg.len. 9.1
## train
PPCR: 0.938651
### report
macro
{'f1-score': 0.6687299945870242,
 'precision': 0.7536596878574793,
 'recall': 0.637184212884146,
 'support': 152238}
micro
{'f1-score': 0.9677478684691075,
 'precision': 0.9677478684691075,
 'recall': 0.9677478684691075,
 'support': 152238}
weighted
{'f1-score': 0.9651372305326062,
 'precision': 0.9649065076775044,
 'recall': 0.9677478684691075,
 'support': 152238}
### report_full
macro
{'f1-score': 0.5507511705566569,
 'precision': 0.7536596878574793,
 'recall': 0.487676648772734,
 'support': 162188}
micro
{'f1-score': 0.9371235203195665,
 'precision': 0.9677478684691075,
 'recall': 0.9083779317828693,
 'support': 162188}
weighted
{'f1-score': 0.9229502599529893,
 'precision': 0.9571981630287524,
 'recall': 0.9083779317828693,
 'support': 162188}
## test
PPCR: 0.934311
### report
macro
{'f1-score': 0.60865785593874,
 'precision': 0.6397286335230646,
 'recall': 0.5989095460784716,
 'support': 32827}
micro
{'f1-score': 0.9107746671946874,
 'precision': 0.9107746671946872,
 'recall': 0.9107746671946872,
 'support': 32827}
weighted
{'f1-score': 0.8981161534999219,
 'precision': 0.887871674779215,
 'recall': 0.9107746671946872,
 'support': 32827}
### report_full
macro
{'f1-score': 0.5078698422635205,
 'precision': 0.6397286335230646,
 'recall': 0.4733696428050167,
 'support': 35135}
micro
{'f1-score': 0.8798446190518232,
 'precision': 0.9107746671946872,
 'recall': 0.850946349793653,
 'support': 35135}
weighted
{'f1-score': 0.8521821725488543,
 'precision': 0.8754795112633311,
 'recall': 0.850946349793653,
 'support': 35135}
```

## javascript
### Summary
30 rules, avg.len. 8.4

| | |
|-|-|
|Min support|110|
|Max support|21222|
|Min confidence|0.9224697947502136|
|Max confidence|0.9997705221176147|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>	∧ -5.diff_offset ≥ 16<br>⇒ y = '<br>Confidence: 0.988. Support: 9951.` |
| 2 | `  -1.roles in {STRING}<br>	∧ -4.roles not in {KEY}<br>	∧ -5.diff_offset ≤ 15<br>⇒ y = '<br>Confidence: 0.958. Support: 848.` |
| 3 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.998. Support: 4125.` |
| 4 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {'}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles in {VALUE}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 1.000. Support: 2179.` |
| 5 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>} and not in {'}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.980. Support: 779.` |
| 6 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<newline>} and not in {', <space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.968. Support: 295.` |
| 7 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {', <newline>, <space>}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.993. Support: 200.` |
| 8 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {', <newline>, <space>}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -4.diff_offset ≥ 10<br>	∧ -5.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.977. Support: 10078.` |
| 9 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 21222.` |
| 10 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 607.` |
| 11 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 178.` |
| 12 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 8674.` |
| 13 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 195.` |
| 14 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 1077.` |
| 15 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1813.` |
| 16 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved not in {function}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 17420.` |
| 17 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 6097.` |
| 18 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4692.` |
| 19 | `  •••start_col ≤ 12<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.966. Support: 781.` |
| 20 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 2610.` |
| 21 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.997. Support: 168.` |
| 22 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<-space>} and not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 229.` |
| 23 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1214.` |
| 24 | `  -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1005.` |
| 25 | `  -1.diff_col ≥ 9<br>	∧ -1.diff_offset ≥ 10<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 210.` |
| 26 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 603.` |
| 27 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 379.` |
| 28 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 14454.` |
| 29 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 168.` |
| 30 | `  -1.diff_col = 0<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 110.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.4, "max_conf": 0.9997705221176147, "max_support": 21222, "min_conf": 0.9224697947502136, "min_support": 110, "num_rules": 30}}
```
</details>
