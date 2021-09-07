# Model report for file:///tmp/top-repos-quality-repos-fbchg_em/whats-new-in-python3.git HEAD 09af9d8a0f446f384866bb06b40fd9e26b9a115d

### Dump

```json
{'created_at': '2021-09-01 03:51:50',
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
 'size': '19.5 kB',
 'tags': [],
 'uuid': '41ad50d5-4839-40db-939c-1a29f1eea858',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-fbchg_em/whats-new-in-python3.git 09af9d8a0f446f384866bb06b40fd9e26b9a115d

# javascript
49 rules, avg.len. 8.5
## train
PPCR: 0.845967
### report
macro
{'f1-score': 0.7708136510457388,
 'precision': 0.7934023420740751,
 'recall': 0.7635497218810134,
 'support': 54575}
micro
{'f1-score': 0.9509482363719652,
 'precision': 0.9509482363719652,
 'recall': 0.9509482363719652,
 'support': 54575}
weighted
{'f1-score': 0.9496125760913698,
 'precision': 0.9502129345450847,
 'recall': 0.9509482363719652,
 'support': 54575}
### report_full
macro
{'f1-score': 0.6649771714183119,
 'precision': 0.7934023420740751,
 'recall': 0.600491576218088,
 'support': 64512}
micro
{'f1-score': 0.8715980753566721,
 'precision': 0.9509482363719652,
 'recall': 0.8044704861111112,
 'support': 64512}
weighted
{'f1-score': 0.8625719957602818,
 'precision': 0.9431564032354272,
 'recall': 0.8044704861111112,
 'support': 64512}
## test
PPCR: 0.893305
### report
macro
{'f1-score': 0.5622453441384738,
 'precision': 0.5486757573687626,
 'recall': 0.5871734574111335,
 'support': 427}
micro
{'f1-score': 0.9297423887587822,
 'precision': 0.9297423887587822,
 'recall': 0.9297423887587822,
 'support': 427}
weighted
{'f1-score': 0.9246716829461704,
 'precision': 0.9231610118049456,
 'recall': 0.9297423887587822,
 'support': 427}
### report_full
macro
{'f1-score': 0.5016047483342942,
 'precision': 0.5486757573687626,
 'recall': 0.4911696970664453,
 'support': 478}
micro
{'f1-score': 0.8773480662983427,
 'precision': 0.9297423887587822,
 'recall': 0.8305439330543933,
 'support': 478}
weighted
{'f1-score': 0.8635623679055173,
 'precision': 0.9124276736583639,
 'recall': 0.8305439330543933,
 'support': 478}
```

## javascript
### Summary
25 rules, avg.len. 8.4

| | |
|-|-|
|Min support|92|
|Max support|9183|
|Min confidence|0.9314835667610168|
|Max confidence|0.9991776347160339|

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
| 1 | `  +1.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 9183.` |
| 2 | `  -1.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.931. Support: 883.` |
| 3 | `  •••start_line ≤ 188<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.950. Support: 131.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {FOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 97.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 402.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.reserved = if<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 251.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.reserved = if<br>	∧ -3.reserved not in {if}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 302.` |
| 8 | `  •••start_line ≥ 254<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.diff_col ≥ 6<br>	∧ -2.reserved not in {if}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 12<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 105.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_col ≥ 5<br>	∧ -3.reserved not in {if}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 5039.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.length ≥ 3<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {if}<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 92.` |
| 11 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 2703.` |
| 12 | `  +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1627.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ -3.reserved = (<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 498.` |
| 14 | `  •••start_line ≤ 191<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.972. Support: 230.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1236.` |
| 16 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = function<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 130.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 608.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 306.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved = '<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 568.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 399.` |
| 21 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {', (}<br>	∧ -5.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 95.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 619.` |
| 23 | `  •••start_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 5472.` |
| 24 | `  •••start_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1718.` |
| 25 | `  •••start_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 5<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 785.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.36, "max_conf": 0.9991776347160339, "max_support": 9183, "min_conf": 0.9314835667610168, "min_support": 92, "num_rules": 25}}
```
</details>
