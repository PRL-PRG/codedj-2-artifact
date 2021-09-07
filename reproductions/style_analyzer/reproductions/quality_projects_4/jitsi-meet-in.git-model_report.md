# Model report for file:///tmp/top-repos-quality-repos-k435anw_/jitsi-meet-in.git HEAD cd2aa1f9628a88ff1a0d1b640f24808e5095ad24

### Dump

```json
{'created_at': '2021-09-02 18:13:59',
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
 'size': '26.2 kB',
 'tags': [],
 'uuid': '2f9a6a7d-a0a7-4b79-a29a-95a559494278',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-k435anw_/jitsi-meet-in.git cd2aa1f9628a88ff1a0d1b640f24808e5095ad24

# javascript
99 rules, avg.len. 11.4
## train
PPCR: 0.946367
### report
macro
{'f1-score': 0.5028527613079041,
 'precision': 0.5111842431341388,
 'recall': 0.49598184688838853,
 'support': 245710}
micro
{'f1-score': 0.9643889137601237,
 'precision': 0.9643889137601237,
 'recall': 0.9643889137601237,
 'support': 245710}
weighted
{'f1-score': 0.9616159657560434,
 'precision': 0.9597161000106366,
 'recall': 0.9643889137601237,
 'support': 245710}
### report_full
macro
{'f1-score': 0.475556507794885,
 'precision': 0.5111842431341388,
 'recall': 0.448600588086281,
 'support': 259635}
micro
{'f1-score': 0.9378147602133196,
 'precision': 0.9643889137601237,
 'recall': 0.9126658578388892,
 'support': 259635}
weighted
{'f1-score': 0.9313632883475553,
 'precision': 0.9555290613043802,
 'recall': 0.9126658578388892,
 'support': 259635}
## test
PPCR: 0.944459
### report
macro
{'f1-score': 0.49880686764435833,
 'precision': 0.5074044429370013,
 'recall': 0.49207174068469445,
 'support': 61540}
micro
{'f1-score': 0.961667208319792,
 'precision': 0.961667208319792,
 'recall': 0.961667208319792,
 'support': 61540}
weighted
{'f1-score': 0.9584341447029358,
 'precision': 0.9562945747708231,
 'recall': 0.961667208319792,
 'support': 61540}
### report_full
macro
{'f1-score': 0.4712761545415705,
 'precision': 0.5074044429370013,
 'recall': 0.4441606922427752,
 'support': 65159}
micro
{'f1-score': 0.9341983756777874,
 'precision': 0.961667208319792,
 'recall': 0.9082551911478076,
 'support': 65159}
weighted
{'f1-score': 0.927392050254023,
 'precision': 0.9522475869478908,
 'recall': 0.9082551911478076,
 'support': 65159}
```

## javascript
### Summary
69 rules, avg.len. 11.0

| | |
|-|-|
|Min support|90|
|Max support|25330|
|Min confidence|0.9206973910331726|
|Max confidence|0.9999219179153442|

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
| 1 | `  +1.internal_type = StringLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 363.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 193.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 298.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 22515.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 6403.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.921. Support: 889.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 6323.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 5123.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.938. Support: 1067.` |
| 10 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 92.` |
| 11 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 3149.` |
| 12 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 196.` |
| 13 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.976. Support: 1250.` |
| 14 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.reserved = )<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.959. Support: 1196.` |
| 15 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved = )<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 253.` |
| 16 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 501.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 612.` |
| 18 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_line = 0<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 359.` |
| 19 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 256.` |
| 20 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, <}<br>	∧ -2.reserved = (<br>	∧ -2.roles not in {IMPORT}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 286.` |
| 21 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.reserved not in {)}<br>	∧ -4.reserved = =<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.993. Support: 201.` |
| 22 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved = else<br>	∧ -4.reserved not in {=}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.995. Support: 109.` |
| 23 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≥ 3<br>	∧ -4.reserved not in {=}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 134.` |
| 24 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {else}<br>	∧ -5.diff_line = 0<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.internal_type = SwitchStatement<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.968. Support: 109.` |
| 25 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_offset ≥ 6<br>	∧ -3.reserved not in {)}<br>	∧ -4.reserved not in {=}<br>	∧ -5.diff_line = 0<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 1768.` |
| 26 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, <, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -3.reserved not in {)}<br>	∧ -4.reserved not in {=}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 18913.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 490.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 328.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 10092.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.970. Support: 627.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.length ≤ 2<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 211.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.length ≤ 1<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {;}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 3825.` |
| 33 | `  •••start_col ≤ 36<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.943. Support: 3462.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 7085.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {<}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 10<br>	∧ ^1.internal_type = JSXOpeningElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 247.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 10<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 8237.` |
| 37 | `  -1.internal_type = CommentBlock<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4147.` |
| 38 | `  -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved = case<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 123.` |
| 39 | `  •••start_col ≥ 2<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type = CommentBlock<br>	∧ +2.length ≥ 22<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.981. Support: 3381.` |
| 40 | `  •••start_col ≤ 1<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type = CommentBlock<br>⇒ y = ∅<br>Confidence: 0.982. Support: 137.` |
| 41 | `  -1.diff_col ≥ 31<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 3011.` |
| 42 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {STATEMENT} and not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.932. Support: 598.` |
| 43 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {SWITCH} and not in {QUALIFIED, STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.995. Support: 96.` |
| 44 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.length ≤ 1<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 180.` |
| 45 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.length ≤ 1<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 5305.` |
| 46 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.length ≤ 1<br>	∧ -3.diff_offset ≤ 5<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 47 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.999. Support: 809.` |
| 48 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 742.` |
| 49 | `  •••start_col ≤ 35<br>	∧ -1.diff_col ≤ 30<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col ≥ 1<br>	∧ -3.diff_offset ≤ 11<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {FUNCTION, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.971. Support: 298.` |
| 50 | `  •••start_col ≤ 35<br>	∧ -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col ≥ 1<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.reserved = (<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 6<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {FUNCTION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 130.` |
| 51 | `  •••start_col ≤ 35<br>	∧ -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col ≥ 1<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 6<br>	∧ +3.reserved not in {{}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {FUNCTION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 631.` |
| 52 | `  •••start_col ≤ 35<br>	∧ -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col ≥ 1<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 6<br>	∧ +3.reserved not in {{}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {FUNCTION, INCOMPLETE, QUALIFIED}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 195.` |
| 53 | `  •••start_col ≤ 35<br>	∧ -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col ≥ 1<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 6<br>	∧ +3.reserved not in {{}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {FUNCTION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 370.` |
| 54 | `  -1.diff_col ≤ 30<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col = 0<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 9<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1188.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1335.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 490.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 430.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, if, return}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CONDITION}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 116.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, if, return}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {CONDITION}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 90.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, if, return}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {CONDITION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 177.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = export<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 192.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, return}<br>	∧ -3.reserved = return<br>	∧ +1.reserved = <<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.998. Support: 277.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return, {}<br>	∧ -3.reserved = return<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 623.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return}<br>	∧ -2.reserved = [<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return, {}<br>	∧ -3.diff_line ≥ 2<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 607.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return, {}<br>	∧ -2.diff_col ≥ 46<br>	∧ -2.label not in {<+space>}<br>	∧ -3.diff_line ≤ 1<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 712.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≤ 45<br>	∧ -3.diff_line ≤ 1<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 25330.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, export, if, return, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 45<br>	∧ -3.diff_line ≤ 1<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 17467.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 795.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 11.014492753623188, "max_conf": 0.9999219179153442, "max_support": 25330, "min_conf": 0.9206973910331726, "min_support": 90, "num_rules": 69}}
```
</details>
