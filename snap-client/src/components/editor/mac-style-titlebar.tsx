interface ButtonProps {
  color: string;
}

const Button = ({ color }: ButtonProps) => (
  <div
    className="w-4 h-4 mr-2 rounded-full"
    style={{ backgroundColor: color }}
  ></div>
);

export const MacStyleTitleBar = () => (
  <div className="flex flex-row">
    <Button color="#FF5E57" />
    <Button color="#FFBC2E" />
    <Button color="#2BC841" />
  </div>
);
