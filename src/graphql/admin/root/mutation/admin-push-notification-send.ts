import { GT } from "@graphql/index"

import AdminPushNotificationSendPayload from "@graphql/admin/types/payload/admin-push-notification-send"
import { Admin } from "@app"
import { mapAndParseErrorForGqlResponse } from "@graphql/error-map"

const AdminPushNotificationSendInput = GT.Input({
  name: "AdminPushNotificationSendInput",
  fields: () => ({
    accountId: {
      type: GT.String,
    },
    title: {
      type: GT.String,
    },
    body: {
      type: GT.String,
    },
  }),
})

const AdminPushNotificationSendMutation = GT.Field<
  {
    input: { accountId: string; title: string; body: string }
  },
  null,
  GraphQLContextAuth
>({
  extensions: {
    complexity: 120,
  },
  type: GT.NonNull(AdminPushNotificationSendPayload),
  args: {
    input: { type: GT.NonNull(AdminPushNotificationSendInput) },
  },
  resolve: async (_, args) => {
    const { accountId, body, title } = args.input

    const success = await Admin.sendAdminPushNotification({ accountId, title, body })

    if (success instanceof Error) {
      return { errors: [mapAndParseErrorForGqlResponse(success)] }
    }
    return { errors: [], success: true }
  },
})

export default AdminPushNotificationSendMutation