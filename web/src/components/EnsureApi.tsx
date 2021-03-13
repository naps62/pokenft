import React, { useEffect, useState, FC } from "react";

import { load, loadContract, ApiContext } from "../lib/api";

const EnsureApi: FC = (props) => {
  const [ctx, setCtx] = useState<ApiContext | undefined>();

  useEffect(() => {
    load().then((api) => {
      const nft = loadContract(api);

      setCtx({ api, nft });
    });
  }, []);

  if (!!ctx) {
    const childrenWithProps = React.Children.map(props.children, (child) => {
      if (React.isValidElement(child)) {
        return React.cloneElement(child, { ctx });
      }
      return child;
    });

    return <>{childrenWithProps}</>;
  } else {
    return <div>Loading</div>;
  }
};

export default EnsureApi;
