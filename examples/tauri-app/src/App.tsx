import { Alert, Button, Input, Space, Table, Typography } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useEffect, useState } from "react";
import {
  type Face,
  getFaces,
  getFamilies,
  isFontgresError,
  queryFace,
  reload,
} from "tauri-plugin-fontgres-api";

const errorText = (error: unknown) =>
  isFontgresError(error) ? `${error.kind}: ${error.message}` : String(error);

const columns: ColumnsType<Face> = [
  {
    title: "Family",
    dataIndex: "family",
    render: (family: string, face) => (
      <span
        style={{
          fontFamily: `"${family}"`,
          fontWeight: face.weight,
          fontStyle: face.style,
          fontStretch: face.stretch,
        }}
      >
        {family}
      </span>
    ),
  },
  { title: "Subfamily", dataIndex: "subfamily" },
  { title: "PostScript name", dataIndex: "postScriptName" },
  { title: "Weight", dataIndex: "weight" },
  { title: "Style", dataIndex: "style" },
  { title: "Stretch", dataIndex: "stretch" },
  { title: "Monospaced", dataIndex: "monospaced", render: String },
  { title: "Path", dataIndex: "path" },
  { title: "Index", dataIndex: "index" },
];

const App = () => {
  const [faces, setFaces] = useState<Face[]>();
  const [familyCount, setFamilyCount] = useState(0);
  const [query, setQuery] = useState("Helvetica Neue, sans-serif");
  const [best, setBest] = useState<string>();
  const [error, setError] = useState<string>();

  const load = async () => {
    setFaces(undefined);
    try {
      setFaces(await getFaces());
      setFamilyCount((await getFamilies()).length);
    } catch (error) {
      setError(errorText(error));
    }
  };

  useEffect(() => {
    void load();
  }, []);

  const runQuery = async () => {
    const families = query.split(",").map((family) => family.trim()).filter(Boolean);
    try {
      const face = await queryFace({ families });
      setBest(`${face.family} ${face.subfamily} (${face.postScriptName})`);
    } catch (error) {
      setBest(errorText(error));
    }
  };

  return (
    <Space orientation="vertical" style={{ width: "100%", padding: 16 }}>
      {error && <Alert type="error" title={error} />}
      <Space>
        <Typography.Text>
          {faces ? `${faces.length} faces in ${familyCount} families` : "Loading…"}
        </Typography.Text>
        <Button
          onClick={async () => {
            await reload();
            await load();
          }}
        >
          Reload
        </Button>
      </Space>
      <Space.Compact style={{ width: "100%" }}>
        <Input
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          onPressEnter={runQuery}
          placeholder="Families in priority order, comma-separated"
        />
        <Button type="primary" onClick={runQuery}>
          Query
        </Button>
      </Space.Compact>
      {best && <Typography.Text>{best}</Typography.Text>}
      <Table
        bordered
        size="small"
        loading={!faces}
        rowKey={(face) => `${face.path}#${face.index}#${face.postScriptName}`}
        columns={columns}
        dataSource={faces}
      />
    </Space>
  );
};

export default App;
