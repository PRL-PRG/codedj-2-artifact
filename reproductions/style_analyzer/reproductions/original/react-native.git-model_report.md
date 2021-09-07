# Model report for file:///tmp/top-repos-quality-repos-92qzanha/react-native.git HEAD 1bc885b8b856c7a050f0df68d9a09ca7581d0220

### Dump

```json
{'created_at': '2021-09-01 23:50:18',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '27.8 kB',
 'tags': [],
 'uuid': '5483e5d8-aa1f-47ec-a150-2c50b48d32ae',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-92qzanha/react-native.git 1bc885b8b856c7a050f0df68d9a09ca7581d0220

# javascript
202 rules, avg.len. 11.4
## train
PPCR: 0.934041
### report
macro
{'f1-score': 0.5716772936302753,
 'precision': 0.6454945244302185,
 'recall': 0.5530760625973049,
 'support': 519790}
micro
{'f1-score': 0.9624309817426269,
 'precision': 0.9624309817426269,
 'recall': 0.9624309817426269,
 'support': 519790}
weighted
{'f1-score': 0.9602299689719817,
 'precision': 0.9613042345265261,
 'recall': 0.9624309817426269,
 'support': 519790}
### report_full
macro
{'f1-score': 0.5147011735613105,
 'precision': 0.6454945244302185,
 'recall': 0.4663836075329615,
 'support': 556496}
micro
{'f1-score': 0.9296079294908604,
 'precision': 0.9624309817426269,
 'recall': 0.8989498576809177,
 'support': 556496}
weighted
{'f1-score': 0.9220471101291949,
 'precision': 0.958531605670936,
 'recall': 0.8989498576809177,
 'support': 556496}
## test
PPCR: 0.932817
### report
macro
{'f1-score': 0.5702922591521303,
 'precision': 0.6358119060539248,
 'recall': 0.557345314126324,
 'support': 102469}
micro
{'f1-score': 0.9637548917233504,
 'precision': 0.9637548917233505,
 'recall': 0.9637548917233505,
 'support': 102469}
weighted
{'f1-score': 0.9611610085469409,
 'precision': 0.9624891487864086,
 'recall': 0.9637548917233505,
 'support': 102469}
### report_full
macro
{'f1-score': 0.5145193001342359,
 'precision': 0.6358119060539248,
 'recall': 0.4703201599826349,
 'support': 109849}
micro
{'f1-score': 0.9302555600561423,
 'precision': 0.9637548917233505,
 'recall': 0.8990068184507825,
 'support': 109849}
weighted
{'f1-score': 0.9217289024823995,
 'precision': 0.9592305763515058,
 'recall': 0.8990068184507825,
 'support': 109849}
```

## javascript
### Summary
122 rules, avg.len. 11.1

| | |
|-|-|
|Min support|90|
|Max support|36112|
|Min confidence|0.9202412962913513|
|Max confidence|0.9999231696128845|

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
| 1 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 2341.` |
| 2 | `  •••start_line ≥ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.999. Support: 801.` |
| 3 | `  •••start_line ≥ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -5.label in {<space>}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.935. Support: 100.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 902.` |
| 5 | `  •••start_line ≥ 249<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {LEFT, STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.991. Support: 277.` |
| 6 | `  •••start_line ≤ 248<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.961. Support: 550.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -1.roles in {LEFT}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 622.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (, {}<br>	∧ -1.length ≥ 2<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 117.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (, {}<br>	∧ -1.roles not in {LEFT}<br>	∧ -1.length ≥ 2<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 166.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (, {}<br>	∧ -1.roles not in {LEFT}<br>	∧ -1.length ≤ 2<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 684.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR, OR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 573.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {BITWISE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 170.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {OPERATOR} and not in {BITWISE}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 279.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {BITWISE, OPERATOR} and not in {OR}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {BITWISE, OPERATOR}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1705.` |
| 17 | `  •••start_line ≥ 58<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {INCOMPLETE}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 36112.` |
| 18 | `  •••start_line ≤ 57<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {INCOMPLETE}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 1097.` |
| 19 | `  •••start_line ≤ 57<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {INCOMPLETE}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 730.` |
| 20 | `  •••start_line ≤ 57<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {INCOMPLETE}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 21 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 275.` |
| 22 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 8<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.941. Support: 111.` |
| 23 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≤ 8<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.977. Support: 6359.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 6510.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 746.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 389.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 768.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 865.` |
| 30 | `  -1.internal_type = DirectiveLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.976. Support: 270.` |
| 31 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.957. Support: 993.` |
| 32 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 15431.` |
| 33 | `  •••start_line = 255<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 300.` |
| 34 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 2249.` |
| 35 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 18692.` |
| 36 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.950. Support: 229.` |
| 37 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 30923.` |
| 38 | `  •••start_line ≤ 254<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.982. Support: 2826.` |
| 39 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = SwitchStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.979. Support: 446.` |
| 40 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ASSIGNMENT}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {ForStatement, IfStatement, SwitchStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 1830.` |
| 41 | `  •••start_col ≥ 25<br>	∧ •••start_line ≥ 237<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 18<br>	∧ -2.roles not in {ASSIGNMENT}<br>	∧ -3.reserved not in {)}<br>	∧ -3.roles not in {ASSIGNMENT, CALL}<br>	∧ -4.reserved = return<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ -5.diff_offset ≤ 47<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IF, OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 126.` |
| 42 | `  •••start_col ≤ 24<br>	∧ •••start_line ≥ 237<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = )<br>	∧ -2.roles not in {ASSIGNMENT}<br>	∧ -3.reserved not in {)}<br>	∧ -3.roles not in {ASSIGNMENT}<br>	∧ -5.diff_offset ≤ 47<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 135.` |
| 43 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles in {PATHNAME}<br>	∧ +1.reserved not in {import, }}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = File<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.924. Support: 217.` |
| 44 | `  •••start_col ≤ 14<br>	∧ •••start_line ≤ 254<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ForStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 210.` |
| 45 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.983. Support: 1878.` |
| 46 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.979. Support: 359.` |
| 47 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 4353.` |
| 48 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 5127.` |
| 49 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = :<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 2963.` |
| 50 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ -5.diff_offset ≤ 14<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = :<br>	∧ +3.roles in {LITERAL}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 363.` |
| 51 | `  •••start_line ≥ 222<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 383.` |
| 52 | `  •••start_line ≥ 222<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT, IDENTIFIER}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {:}<br>	∧ +2.length ≤ 25<br>	∧ +3.reserved = .<br>	∧ ^1.roles not in {BODY, CALL, LITERAL, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 417.` |
| 53 | `  •••start_line ≤ 221<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 355.` |
| 54 | `  •••start_line ≤ 221<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = File<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.994. Support: 90.` |
| 55 | `  •••start_line ≤ 221<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.roles in {ARGUMENT} and not in {VALUE}<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 964.` |
| 56 | `  •••start_line ≤ 221<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT, EXPRESSION}<br>	∧ -3.roles not in {ARGUMENT, VALUE}<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {LITERAL}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 147.` |
| 57 | `  -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +2.roles in {CASE}<br>	∧ ^1.roles in {SWITCH} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 475.` |
| 58 | `  -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {CASE}<br>	∧ ^1.roles in {SWITCH} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.999. Support: 1111.` |
| 59 | `  -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 12972.` |
| 60 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 3<br>	∧ -3.diff_offset ≥ 180<br>	∧ -3.length ≥ 93<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.974. Support: 254.` |
| 61 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 3<br>	∧ -3.diff_offset ≥ 180<br>	∧ -3.length ≤ 93<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 258.` |
| 62 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = StringLiteralTypeAnnotation<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 183.` |
| 63 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteralTypeAnnotation}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 6216.` |
| 64 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 1226.` |
| 65 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≥ 20<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 105.` |
| 66 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 19<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 339.` |
| 67 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 19<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 2381.` |
| 68 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {STATEMENT} and not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 639.` |
| 69 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 7<br>	∧ -2.label in {<newline>}<br>	∧ -5.roles not in {ASSIGNMENT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1408.` |
| 70 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION, STATEMENT}<br>	∧ -1.length ≥ 7<br>	∧ -2.label in {<newline>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 116.` |
| 71 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.label in {<newline>}<br>	∧ -5.roles not in {ASSIGNMENT}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 15612.` |
| 72 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, ;, function}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<space>} and not in {<newline>}<br>	∧ -3.reserved = :<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 73 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved not in {,, ;, function}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<space>} and not in {<newline>}<br>	∧ -3.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 3192.` |
| 74 | `  •••start_line ≤ 254<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +2.reserved = ,<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.970. Support: 419.` |
| 75 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<newline>}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.length ≥ 114<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 182.` |
| 76 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<newline>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.label in {<space>}<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 119.` |
| 77 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<newline>}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.label not in {<space>}<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 160.` |
| 78 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<newline>}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 1927.` |
| 79 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.992. Support: 315.` |
| 80 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved = throw<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 58<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 227.` |
| 81 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≥ 4<br>	∧ -4.reserved not in {throw}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.996. Support: 140.` |
| 82 | `  •••start_line = 255<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, =}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = return<br>	∧ -4.reserved not in {throw}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = "<br>Confidence: 0.985. Support: 97.` |
| 83 | `  •••start_line = 255<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, =}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {return}<br>	∧ -3.diff_col ≤ 17<br>	∧ -3.length ≤ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = '<br>Confidence: 0.938. Support: 283.` |
| 84 | `  •••start_line ≤ 254<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, =}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 10<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {throw}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved = )<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 117.` |
| 85 | `  •••start_line ≤ 254<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, =}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {throw}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.964. Support: 5253.` |
| 86 | `  •••start_line ≥ 16<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.983. Support: 7538.` |
| 87 | `  •••start_col ≥ 26<br>	∧ •••start_line ≤ 15<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.975. Support: 102.` |
| 88 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 2506.` |
| 89 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.925. Support: 379.` |
| 90 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {=}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.981. Support: 5879.` |
| 91 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, =, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {=}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type = Identifier<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.993. Support: 224.` |
| 92 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {=}<br>	∧ -5.diff_offset ≤ 5<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.974. Support: 98.` |
| 93 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2011.` |
| 94 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles in {MAP}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 651.` |
| 95 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {MAP}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 6293.` |
| 96 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 1880.` |
| 97 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {MODULE} and not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 342.` |
| 98 | `  •••start_line ≥ 233<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, else}<br>	∧ +1.length ≥ 2<br>	∧ +4.roles in {ARGUMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {EXPRESSION, MODULE, SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.975. Support: 906.` |
| 99 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 205.` |
| 100 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 5<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 214.` |
| 101 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 4<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 6100.` |
| 102 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1138.` |
| 103 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 269.` |
| 104 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {(, ), ,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1204.` |
| 105 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {(, ), ,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 233.` |
| 106 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 6<br>	∧ +2.reserved = (<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 228.` |
| 107 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 5<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.977. Support: 577.` |
| 108 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.935. Support: 178.` |
| 109 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 238.` |
| 110 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles in {BINARY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.926. Support: 102.` |
| 111 | `  •••start_line ≥ 139<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {BINARY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.962. Support: 250.` |
| 112 | `  -1.diff_col ≤ 1<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type = UnionTypeAnnotation<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.997. Support: 155.` |
| 113 | `  •••start_col ≥ 44<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +3.length ≥ 14<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.929. Support: 190.` |
| 114 | `  •••start_col ≤ 36<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 14<br>	∧ +3.length ≤ 13<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 105.` |
| 115 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved not in {,}<br>	∧ +4.roles in {IMPORT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 124.` |
| 116 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles not in {ASSIGNMENT}<br>	∧ +2.length ≥ 14<br>	∧ +3.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 95.` |
| 117 | `  •••start_col ≥ 70<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {ASSIGNMENT}<br>	∧ +2.length ≤ 1<br>	∧ +3.length ≤ 19<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {AssignmentExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 181.` |
| 118 | `  •••start_col ≤ 69<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {ASSIGNMENT}<br>	∧ +2.length ≤ 1<br>	∧ +3.length ≤ 19<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {AssignmentExpression, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 15950.` |
| 119 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {MODULE}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 147.` |
| 120 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {(, ,, :, ;, =, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +2.reserved not in {;}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 516.` |
| 121 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {(, ,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = NumericLiteral<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {LITERAL}<br>	∧ +2.reserved not in {;}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.internal_type not in {ConditionalExpression, ForStatement, IfStatement}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 364.` |
| 122 | `  -1.diff_col ≤ 1<br>	∧ -1.reserved not in {(, ,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {;}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.internal_type not in {ConditionalExpression, ForStatement, IfStatement}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 30632.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 11.12295081967213, "max_conf": 0.9999231696128845, "max_support": 36112, "min_conf": 0.9202412962913513, "min_support": 90, "num_rules": 122}}
```
</details>
