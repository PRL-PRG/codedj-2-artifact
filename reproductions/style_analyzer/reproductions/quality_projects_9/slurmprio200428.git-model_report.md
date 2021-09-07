# Model report for file:///tmp/top-repos-quality-repos-_1j9kygo/slurmprio200428.git HEAD 6fd1dead667cb3aa5d5f5bc3d8a2920f396cb643

### Dump

```json
{'created_at': '2021-08-31 21:53:12',
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
 'size': '19.6 kB',
 'tags': [],
 'uuid': 'a4fa65cd-c3bb-49b5-b93a-6bb29e162c77',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_1j9kygo/slurmprio200428.git 6fd1dead667cb3aa5d5f5bc3d8a2920f396cb643

# javascript
50 rules, avg.len. 8.5
## train
PPCR: 0.842146
### report
macro
{'f1-score': 0.7743492837362028,
 'precision': 0.8001197034078112,
 'recall': 0.7663542438657391,
 'support': 54854}
micro
{'f1-score': 0.9543333211798593,
 'precision': 0.9543333211798593,
 'recall': 0.9543333211798593,
 'support': 54854}
weighted
{'f1-score': 0.9531847822808623,
 'precision': 0.9537861910547277,
 'recall': 0.9543333211798593,
 'support': 54854}
### report_full
macro
{'f1-score': 0.6650764793686268,
 'precision': 0.8001197034078112,
 'recall': 0.608011883102759,
 'support': 65136}
micro
{'f1-score': 0.8725560463371947,
 'precision': 0.9543333211798593,
 'recall': 0.8036876688774257,
 'support': 65136}
weighted
{'f1-score': 0.8630739842584209,
 'precision': 0.9477015351475445,
 'recall': 0.8036876688774257,
 'support': 65136}
## test
PPCR: 0.870293
### report
macro
{'f1-score': 0.5710924452224762,
 'precision': 0.5573313090418354,
 'recall': 0.5957627118644068,
 'support': 416}
micro
{'f1-score': 0.9326923076923077,
 'precision': 0.9326923076923077,
 'recall': 0.9326923076923077,
 'support': 416}
weighted
{'f1-score': 0.927566159529127,
 'precision': 0.9256181550399667,
 'recall': 0.9326923076923077,
 'support': 416}
### report_full
macro
{'f1-score': 0.5089811677071776,
 'precision': 0.5573313090418354,
 'recall': 0.4946896434291673,
 'support': 478}
micro
{'f1-score': 0.8680089485458613,
 'precision': 0.9326923076923077,
 'recall': 0.8117154811715481,
 'support': 478}
weighted
{'f1-score': 0.8550627515008861,
 'precision': 0.9186194586078973,
 'recall': 0.8117154811715481,
 'support': 478}
```

## javascript
### Summary
28 rules, avg.len. 8.1

| | |
|-|-|
|Min support|92|
|Max support|9213|
|Min confidence|0.9242424368858337|
|Max confidence|0.9997173547744751|

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
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label in {'}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 112.` |
| 2 | `  +1.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 9213.` |
| 3 | `  -1.label in {<space>}<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 612.` |
| 4 | `  -1.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -4.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.971. Support: 190.` |
| 5 | `  •••start_line ≥ 189<br>	∧ -1.label in {<space>}<br>	∧ -5.diff_col ≤ 6<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.924. Support: 99.` |
| 6 | `  •••start_line ≤ 188<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.971. Support: 122.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {FOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 97.` |
| 8 | `  •••start_col ≥ 19<br>	∧ -1.diff_col ≥ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 386.` |
| 9 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.reserved = if<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 265.` |
| 10 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.reserved = if<br>	∧ -3.reserved not in {if}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 326.` |
| 11 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved not in {if}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 392.` |
| 12 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved not in {if}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 5159.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 2657.` |
| 14 | `  +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 1662.` |
| 15 | `  -1.roles in {STRING}<br>	∧ -3.reserved = (<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 491.` |
| 16 | `  •••start_line ≤ 190<br>	∧ -1.roles in {STRING}<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.973. Support: 237.` |
| 17 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1257.` |
| 18 | `  -1.diff_col ≥ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {NAME}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 19 | `  -1.diff_col ≥ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {NAME}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 781.` |
| 20 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 592.` |
| 21 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 623.` |
| 22 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 321.` |
| 23 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -3.label in {'}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 557.` |
| 24 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.label not in {'}<br>	∧ -3.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 386.` |
| 25 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.label not in {'}<br>	∧ -3.reserved not in {(}<br>	∧ -5.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 92.` |
| 26 | `  •••start_col ≥ 12<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5519.` |
| 27 | `  •••start_col ≥ 12<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1769.` |
| 28 | `  •••start_col ≤ 11<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≤ 5<br>	∧ +1.reserved not in {(, ), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 797.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.071428571428571, "max_conf": 0.9997173547744751, "max_support": 9213, "min_conf": 0.9242424368858337, "min_support": 92, "num_rules": 28}}
```
</details>
