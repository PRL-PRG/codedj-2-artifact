# Model report for file:///tmp/top-repos-quality-repos-2ovvvp80/ra16-homeworks.git HEAD fd001024467295eef79c44013c65eaa7838f2d21

### Dump

```json
{'created_at': '2021-09-02 16:02:54',
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
 'size': '19.8 kB',
 'tags': [],
 'uuid': '2f7e8cfd-fa60-41e7-ad01-2164ae3fd97d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2ovvvp80/ra16-homeworks.git fd001024467295eef79c44013c65eaa7838f2d21

# javascript
31 rules, avg.len. 6.7
## train
PPCR: 0.852364
### report
macro
{'f1-score': 0.6927321350213844,
 'precision': 0.7265478439651352,
 'recall': 0.6689188550413336,
 'support': 28538}
micro
{'f1-score': 0.9492956759408508,
 'precision': 0.9492956759408508,
 'recall': 0.9492956759408508,
 'support': 28538}
weighted
{'f1-score': 0.9450627000903978,
 'precision': 0.9424886240872077,
 'recall': 0.9492956759408508,
 'support': 28538}
### report_full
macro
{'f1-score': 0.540700304988989,
 'precision': 0.7265478439651352,
 'recall': 0.46305254804132,
 'support': 33481}
micro
{'f1-score': 0.8736354987987552,
 'precision': 0.9492956759408508,
 'recall': 0.8091454854992384,
 'support': 33481}
weighted
{'f1-score': 0.852342616809298,
 'precision': 0.9237254934935459,
 'recall': 0.8091454854992384,
 'support': 33481}
## test
PPCR: 0.852823
### report
macro
{'f1-score': 0.6863211069107027,
 'precision': 0.7207989544215465,
 'recall': 0.6612919306736288,
 'support': 6345}
micro
{'f1-score': 0.9424743892829,
 'precision': 0.9424743892829,
 'recall': 0.9424743892829,
 'support': 6345}
weighted
{'f1-score': 0.9391761761445164,
 'precision': 0.9379732718130961,
 'recall': 0.9424743892829,
 'support': 6345}
### report_full
macro
{'f1-score': 0.5539757418963521,
 'precision': 0.7207989544215465,
 'recall': 0.48717774248933016,
 'support': 7440}
micro
{'f1-score': 0.8676097207109177,
 'precision': 0.9424743892829,
 'recall': 0.803763440860215,
 'support': 7440}
weighted
{'f1-score': 0.8465329428989669,
 'precision': 0.9208446927158268,
 'recall': 0.803763440860215,
 'support': 7440}
```

## javascript
### Summary
18 rules, avg.len. 6.6

| | |
|-|-|
|Min support|95|
|Max support|5432|
|Min confidence|0.9210526347160339|
|Max confidence|0.99909907579422|

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
| 1 | `  +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.975. Support: 3705.` |
| 2 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.997. Support: 167.` |
| 3 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.921. Support: 95.` |
| 4 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 2169.` |
| 5 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 6 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {:}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 47<br>	∧ ^1.internal_type not in {JSXAttribute, MemberExpression}<br>⇒ y = '<br>Confidence: 0.967. Support: 658.` |
| 7 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 174.` |
| 8 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.998. Support: 885.` |
| 9 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 836.` |
| 10 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 555.` |
| 11 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.996. Support: 135.` |
| 12 | `  •••start_col ≥ 8<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved = <<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.958. Support: 106.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.968. Support: 617.` |
| 14 | `  -1.internal_type = JSXIdentifier<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 327.` |
| 15 | `  -1.internal_type not in {JSXIdentifier, StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 816.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 275.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≤ 37<br>	∧ -4.reserved not in {'}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 360.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, >, if, return}<br>	∧ -2.diff_offset ≤ 37<br>	∧ -4.reserved not in {'}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 5432.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.555555555555555, "max_conf": 0.99909907579422, "max_support": 5432, "min_conf": 0.9210526347160339, "min_support": 95, "num_rules": 18}}
```
</details>
