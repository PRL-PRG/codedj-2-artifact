# Model report for file:///tmp/top-repos-quality-repos-xm99e96q/soundnode-redux.git HEAD 13a69d5bdc21cb8fb369b2a2576fc395ce993d19

### Dump

```json
{'created_at': '2021-08-31 22:57:13',
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
 'size': '17.7 kB',
 'tags': [],
 'uuid': 'fce97781-9dd7-4d0f-8394-8487cd3a7d7c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xm99e96q/soundnode-redux.git 13a69d5bdc21cb8fb369b2a2576fc395ce993d19

# javascript
95 rules, avg.len. 6.5
## train
PPCR: 0.915959
### report
macro
{'f1-score': 0.899289638921114,
 'precision': 0.9309826726726511,
 'recall': 0.8750118223311031,
 'support': 15433}
micro
{'f1-score': 0.9293721246679194,
 'precision': 0.9293721246679194,
 'recall': 0.9293721246679194,
 'support': 15433}
weighted
{'f1-score': 0.9285814517218528,
 'precision': 0.9293769630209401,
 'recall': 0.9293721246679194,
 'support': 15433}
### report_full
macro
{'f1-score': 0.7963456380619857,
 'precision': 0.9309826726726511,
 'recall': 0.7483329499715156,
 'support': 16849}
micro
{'f1-score': 0.8886066538628338,
 'precision': 0.9293721246679194,
 'recall': 0.8512671375155796,
 'support': 16849}
weighted
{'f1-score': 0.8717133019094282,
 'precision': 0.9280682011267335,
 'recall': 0.8512671375155796,
 'support': 16849}
## test
PPCR: 0.924444
### report
macro
{'f1-score': 0.9018326212970599,
 'precision': 0.9155944839973468,
 'recall': 0.8930518183738764,
 'support': 3536}
micro
{'f1-score': 0.9092194570135747,
 'precision': 0.9092194570135747,
 'recall': 0.9092194570135747,
 'support': 3536}
weighted
{'f1-score': 0.9089770208920032,
 'precision': 0.9102580582571959,
 'recall': 0.9092194570135747,
 'support': 3536}
### report_full
macro
{'f1-score': 0.8041801429395574,
 'precision': 0.9155944839973468,
 'recall': 0.7525103785235422,
 'support': 3825}
micro
{'f1-score': 0.8735226192093466,
 'precision': 0.9092194570135747,
 'recall': 0.8405228758169935,
 'support': 3825}
weighted
{'f1-score': 0.8666762847362364,
 'precision': 0.911372447843176,
 'recall': 0.8405228758169935,
 'support': 3825}
```

## javascript
### Summary
60 rules, avg.len. 5.5

| | |
|-|-|
|Min support|137|
|Max support|2798|
|Min confidence|0.9215817451477051|
|Max confidence|0.9989361763000488|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
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
| 1 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 447.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 2605.` |
| 3 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.971. Support: 534.` |
| 4 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -1.length ≥ 4<br>	∧ -4.length ≤ 1<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.969. Support: 409.` |
| 5 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 4<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 1560.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = }<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {}}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.937. Support: 564.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.999. Support: 394.` |
| 9 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 280.` |
| 10 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≤ 3<br>	∧ -3.diff_col ≥ 5<br>	∧ -3.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 838.` |
| 11 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ><br>	∧ -1.length ≤ 3<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 150.` |
| 12 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 383.` |
| 13 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 222.` |
| 14 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 437.` |
| 15 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 2696.` |
| 16 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.991. Support: 525.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 4<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 1597.` |
| 18 | `  •••start_col ≥ 11<br>	∧ -1.diff_offset ≤ 3<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.924. Support: 530.` |
| 19 | `  -1.diff_offset ≤ 3<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.999. Support: 387.` |
| 20 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 263.` |
| 21 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 137.` |
| 22 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 378.` |
| 23 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved = from<br>⇒ y = ␣<br>Confidence: 0.996. Support: 142.` |
| 24 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved not in {from}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 2791.` |
| 25 | `  -1.diff_col ≤ 8<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -1.length ≥ 4<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 1588.` |
| 26 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = }<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 27 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {}}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.923. Support: 539.` |
| 28 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.999. Support: 379.` |
| 29 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 279.` |
| 30 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 31 | `  -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 361.` |
| 32 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 235.` |
| 33 | `  -1.diff_offset ≤ 3<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = }<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 34 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_col ≥ 8<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 746.` |
| 35 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 239.` |
| 36 | `  •••start_col ≥ 32<br>	∧ -1.diff_offset ≥ 4<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.953. Support: 416.` |
| 37 | `  -1.diff_offset ≤ 3<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.999. Support: 379.` |
| 38 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.diff_col ≥ 16<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 728.` |
| 39 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.label in {<space>}<br>	∧ -5.diff_col ≤ 15<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 176.` |
| 40 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 989.` |
| 41 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.975. Support: 506.` |
| 42 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.999. Support: 470.` |
| 43 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≥ 4<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 1651.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.999. Support: 408.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 142.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 366.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 234.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = function<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.958. Support: 155.` |
| 49 | `  •••start_col ≥ 32<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -1.length ≥ 4<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.939. Support: 431.` |
| 50 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -1.length ≥ 4<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 1681.` |
| 51 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.999. Support: 391.` |
| 52 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 258.` |
| 53 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 54 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 4<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 1686.` |
| 55 | `  -1.diff_offset ≤ 3<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.990. Support: 460.` |
| 56 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved = from<br>⇒ y = ␣<br>Confidence: 0.983. Support: 150.` |
| 57 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved not in {from}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 2798.` |
| 58 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 4<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1623.` |
| 59 | `  -1.internal_type not in {Identifier}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.988. Support: 532.` |
| 60 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≤ 3<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.533333333333333, "max_conf": 0.9989361763000488, "max_support": 2798, "min_conf": 0.9215817451477051, "min_support": 137, "num_rules": 60}}
```
</details>
