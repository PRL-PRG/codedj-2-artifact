# Model report for file:///tmp/top-repos-quality-repos-54h1u8ih/kingsland-blockchain-advanced-project.git HEAD 30c7ea23a73509b0abee2d47920032f112a205c3

### Dump

```json
{'created_at': '2021-08-31 19:58:07',
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
 'size': '21.0 kB',
 'tags': [],
 'uuid': '3b8140ca-4b66-44d6-bd35-7147bf385198',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-54h1u8ih/kingsland-blockchain-advanced-project.git 30c7ea23a73509b0abee2d47920032f112a205c3

# javascript
43 rules, avg.len. 9.3
## train
PPCR: 0.951176
### report
macro
{'f1-score': 0.6324575229243141,
 'precision': 0.6733452619305229,
 'recall': 0.6292507921020563,
 'support': 73621}
micro
{'f1-score': 0.9705926298202958,
 'precision': 0.9705926298202958,
 'recall': 0.9705926298202958,
 'support': 73621}
weighted
{'f1-score': 0.9666153966878539,
 'precision': 0.9674716705721483,
 'recall': 0.9705926298202958,
 'support': 73621}
### report_full
macro
{'f1-score': 0.588730764289943,
 'precision': 0.6733452619305229,
 'recall': 0.5584773888177123,
 'support': 77400}
micro
{'f1-score': 0.9463054806947379,
 'precision': 0.9705926298202958,
 'recall': 0.923204134366925,
 'support': 77400}
weighted
{'f1-score': 0.9387252568881341,
 'precision': 0.9647540286029072,
 'recall': 0.923204134366925,
 'support': 77400}
## test
PPCR: 0.858494
### report
macro
{'f1-score': 0.44409119535068914,
 'precision': 0.4671334964059986,
 'recall': 0.5229257341180026,
 'support': 4265}
micro
{'f1-score': 0.853223915592028,
 'precision': 0.8532239155920281,
 'recall': 0.8532239155920281,
 'support': 4265}
weighted
{'f1-score': 0.8383684318350424,
 'precision': 0.8495488689760742,
 'recall': 0.8532239155920281,
 'support': 4265}
### report_full
macro
{'f1-score': 0.3732114556503637,
 'precision': 0.4671334964059986,
 'recall': 0.39253798315035754,
 'support': 4968}
micro
{'f1-score': 0.7882595039532114,
 'precision': 0.8532239155920281,
 'recall': 0.732487922705314,
 'support': 4968}
weighted
{'f1-score': 0.7672884252702518,
 'precision': 0.8452538435775481,
 'recall': 0.732487922705314,
 'support': 4968}
```

## javascript
### Summary
28 rules, avg.len. 9.0

| | |
|-|-|
|Min support|97|
|Max support|13580|
|Min confidence|0.9223107695579529|
|Max confidence|0.9989456534385681|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 13580.` |
| 2 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {ADD, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.981. Support: 297.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.979. Support: 117.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 215.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 5274.` |
| 7 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 1409.` |
| 8 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 8374.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4268.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 2932.` |
| 11 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.945. Support: 1872.` |
| 12 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.953. Support: 1702.` |
| 13 | `  •••start_col ≥ 10<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, if, }}<br>	∧ +4.reserved = =<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 240.` |
| 14 | `  •••start_col ≥ 10<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, if, this, }}<br>	∧ +4.reserved not in {=}<br>	∧ +4.roles in {LITERAL}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 122.` |
| 15 | `  •••start_col ≤ 9<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, if, }}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.990. Support: 251.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {LEFT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.927. Support: 750.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 794.` |
| 18 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≤ 50<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 377.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_col ≥ 113<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 219.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_col ≤ 112<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 126.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 750.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 627.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 169.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {File, ObjectExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR, QUALIFIED, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 126.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {File, ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 251.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {File, ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 8668.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {File, ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 120.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {File, ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 97.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.0, "max_conf": 0.9989456534385681, "max_support": 13580, "min_conf": 0.9223107695579529, "min_support": 97, "num_rules": 28}}
```
</details>
